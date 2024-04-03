use gpui::prelude::FluentBuilder;
use gpui::{
    div, AnyElement, AppContext, ClickEvent, Div, ElementId, InteractiveElement, IntoElement,
    MouseButton, ParentElement, RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled,
    WindowContext,
};
use smallvec::SmallVec;
use theme::ActiveTheme;

use crate::disableable::Disableable;
use crate::selectable::Selectable;

#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct Button {
    base: Div,
    id: ElementId,
    children: SmallVec<[AnyElement; 2]>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    disabled: bool,
    selected: bool,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, cx: &AppContext) -> Self {
        let base = div()
            .border()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .bg(cx.theme().colors().element_background);

        Self {
            base,
            id: id.into(),
            children: SmallVec::new(),
            on_click: None,
            disabled: false,
            selected: false,
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

impl Disableable for Button {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Selectable for Button {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
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

impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        self.base
            .id(self.id.clone())
            .when(self.selected, |this| {
                this.border_color(cx.theme().colors().border_selected)
            })
            .when(self.disabled, |this| {
                this.cursor_not_allowed()
                    .border_color(cx.theme().colors().border)
            })
            .when(!self.disabled, |this| {
                this.cursor_pointer()
                    .hover(|hover| hover.bg(cx.theme().colors().element_background_hover))
                    .active(|active| active.bg(cx.theme().colors().element_background_active))
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
