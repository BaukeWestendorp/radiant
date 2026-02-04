use std::rc::Rc;

use gpui::{
    AnyElement, App, ClickEvent, Div, ElementId, Interactivity, Stateful, StyleRefinement, Window,
    div, prelude::*,
};
use smallvec::SmallVec;

use crate::{ActiveTheme, Icon, theme::HslaExt};

#[derive(IntoElement)]
pub struct Button {
    base: Stateful<Div>,
    style: StyleRefinement,
    disabled: bool,
    selected: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    children: SmallVec<[AnyElement; 1]>,
    icon: Option<Icon>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            base: div().id(id),
            style: StyleRefinement::default(),
            disabled: false,
            selected: false,
            on_click: None,
            children: SmallVec::new(),
            icon: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let clickable = !self.disabled;

        let (bg, border_color, text_color) = if self.selected {
            (cx.theme().bg_selected, cx.theme().border_selected, cx.theme().fg_selected)
        } else {
            (cx.theme().bg_tertiary, cx.theme().border_tertiary, cx.theme().fg_primary)
        };

        self.base
            .flex()
            .items_center()
            .justify_center()
            .px_2()
            .min_size_4()
            .bg(bg)
            .border_color(border_color)
            .border_1()
            .rounded(cx.theme().radius)
            .text_color(text_color)
            .occlude()
            .when(self.disabled, |e| {
                e.bg(bg.disabled())
                    .border_color(border_color.disabled())
                    .text_color(text_color.disabled())
                    .cursor_not_allowed()
            })
            .when(!self.disabled, |e| {
                e.hover(|e| e.bg(bg.hover()).border_color(border_color.hover()))
                    .active(|e| e.bg(bg.active()).border_color(border_color.active()))
            })
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |event, window, cx| {
                    if !clickable {
                        cx.stop_propagation();
                        return;
                    }

                    on_click(event, window, cx);
                })
            })
            .when(cx.theme().shadow, |e| e.shadow_xs())
            .children(self.icon)
            .children(self.children)
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl InteractiveElement for Button {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}
