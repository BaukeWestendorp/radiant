use gpui::prelude::FluentBuilder;
use gpui::{
    div, AnyElement, AnyView, ClickEvent, Div, ElementId, InteractiveElement, IntoElement,
    MouseButton, ParentElement, RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled,
    WindowContext,
};
use smallvec::SmallVec;
use theme::ActiveTheme;

#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct Button {
    base: Div,
    id: ElementId,
    children: SmallVec<[AnyElement; 2]>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    tooltip: Option<Box<dyn Fn(&mut WindowContext) -> AnyView>>,
    disabled: bool,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div(),
            id: id.into(),
            children: SmallVec::new(),
            tooltip: None,
            on_click: None,
            disabled: false,
        }
    }

    pub fn tooltip(mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }

    pub fn on_click(
        mut self,
        listener: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(listener));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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
            .bg(gpui::red())
            .border()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .bg(cx.theme().colors().element_background)
            .when(self.disabled, |this| {
                this.cursor_not_allowed()
                    .border_color(cx.theme().colors().border_variant)
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
            .when_some(self.tooltip, |this, tooltip| {
                this.tooltip(move |cx| tooltip(cx))
            })
            .children(self.children)
    }
}
