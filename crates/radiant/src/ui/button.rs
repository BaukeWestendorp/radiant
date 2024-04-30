use gpui::{
    div, prelude::FluentBuilder, AnyElement, ClickEvent, Div, ElementId, InteractiveElement,
    IntoElement, MouseButton, ParentElement, RenderOnce, StatefulInteractiveElement,
    StyleRefinement, Styled, WindowContext,
};
use smallvec::SmallVec;

use crate::theme::{Activatable, Disableable, Hoverable, THEME};

#[derive(IntoElement)]
pub struct Button {
    base: Div,
    id: ElementId,
    children: SmallVec<[AnyElement; 2]>,
    selected: bool,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div(),
            id: id.into(),
            children: SmallVec::new(),
            selected: false,
            disabled: false,
            on_click: None,
        }
    }

    pub fn on_click(
        mut self,
        listener: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        self.base
            .rounded_md()
            .border()
            .border_color(THEME.border)
            .bg(THEME.fill_secondary)
            .text_color(THEME.text)
            .id(self.id.clone())
            .when(self.selected, |this| {
                this.border_color(THEME.border_selected)
            })
            .when(self.disabled, |this| {
                this.cursor_not_allowed()
                    .bg(THEME.fill_secondary.disabled())
                    .border_color(THEME.border.disabled())
                    .text_color(THEME.text.disabled())
            })
            .when(!self.disabled, |this| {
                this.cursor_pointer()
                    .hover(|hover| hover.bg(THEME.fill_secondary.hovered()))
                    .active(|active| active.bg(THEME.fill_secondary.active()))
            })
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_mouse_down(MouseButton::Left, move |_event, cx| cx.prevent_default())
                        .on_click(move |event, cx| {
                            cx.stop_propagation();
                            (on_click)(event, cx)
                        })
                },
            )
            .children(self.children)
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl super::Selectable for Button {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl super::Disableable for Button {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
