use std::rc::Rc;

use gpui::{
    Action, AnyElement, App, ClickEvent, Div, ElementId, Interactivity, Stateful, StyleRefinement,
    Window, div,
};
use gpui::{prelude::*, px};
use smallvec::SmallVec;

use crate::styled_ext::{FocusableExt, StatefulInteractiveElementExt};
use crate::theme::HslaExt;
use crate::{ActiveTheme, Icon, StyledExt};

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    base: Stateful<Div>,
    style: StyleRefinement,
    disabled: bool,
    focusable: bool,
    selected: bool,
    tab_index: isize,
    tab_stop: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    action: Option<Box<dyn Action>>,
    children: SmallVec<[AnyElement; 2]>,
    icon: Option<Icon>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            id: id.clone(),
            base: div().id(id),
            style: StyleRefinement::default(),
            disabled: false,
            focusable: true,
            selected: false,
            tab_index: 0,
            tab_stop: true,

            on_click: None,
            action: None,
            children: SmallVec::new(),
            icon: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = focusable;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn tab_index(mut self, tab_index: isize) -> Self {
        self.tab_index = tab_index;
        self
    }

    pub fn tab_stop(mut self, tab_stop: bool) -> Self {
        self.tab_stop = tab_stop;
        self
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn action(mut self, action: impl Action) -> Self {
        self.action = Some(Box::new(action));
        self
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let clickable = !self.disabled;

        let (bg, border_color, text_color) = if self.selected {
            (cx.theme().bg_selected, cx.theme().border_selected, cx.theme().fg_selected)
        } else {
            (cx.theme().bg_tertiary, cx.theme().border_tertiary, cx.theme().fg_primary)
        };

        let focus_handle =
            window.use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle()).read(cx).clone();
        let is_focused = self.focusable && focus_handle.is_focused(window);

        self.base
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .px_2()
            .when(self.icon.is_some() && self.children.is_empty(), |e| e.px_0p5())
            .min_size_2()
            .gap_1()
            .bg(bg)
            .border_color(border_color)
            .border_1()
            .rounded(cx.theme().radius)
            .text_color(text_color)
            .block_mouse_except_scroll()
            .when(self.disabled, |e| {
                e.bg(bg.disabled())
                    .border_color(border_color.disabled())
                    .text_color(text_color.disabled())
                    .cursor_not_allowed()
            })
            .when(!self.disabled, |e| {
                e.hover(|e| e.bg(bg.hover()).border_color(border_color.hover()))
                    .active(|e| {
                        e.bg(bg.active())
                            .border_color(border_color.active())
                            .top(cx.theme().button_depression)
                    })
                    .when(self.focusable, |e| {
                        e.track_focus(
                            &focus_handle.tab_index(self.tab_index).tab_stop(self.tab_stop),
                        )
                    })
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
            .when_some(self.action, |this, action| {
                this.action_tooltip(action.boxed_clone()).on_click(move |_, window, cx| {
                    if !clickable {
                        cx.stop_propagation();
                        return;
                    }

                    window.dispatch_action(action.boxed_clone(), cx);
                })
            })
            .when(cx.theme().shadow, |e| e.shadow_xs())
            .focus_ring(is_focused, px(1.0), window, cx)
            .refine_style(&self.style)
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
