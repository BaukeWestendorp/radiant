use gpui::{
    div, prelude::FluentBuilder, AnyElement, Div, InteractiveElement, IntoElement, MouseDownEvent,
    MouseUpEvent, ParentElement, RenderOnce, StyleRefinement, Styled, WindowContext,
};
use smallvec::SmallVec;
use theme::ActiveTheme;

#[derive(IntoElement)]
pub struct Container {
    base: Div,
    children: SmallVec<[AnyElement; 2]>,
    mouse_down_listener: Option<Box<dyn Fn(&MouseDownEvent, &mut WindowContext) + 'static>>,
    mouse_up_listener: Option<Box<dyn Fn(&MouseUpEvent, &mut WindowContext) + 'static>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            base: div(),
            children: SmallVec::new(),
            mouse_down_listener: None,
            mouse_up_listener: None,
        }
    }

    pub fn on_click_down(
        mut self,
        listener: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.mouse_down_listener = Some(Box::new(listener));
        self
    }

    pub fn on_click_up(
        mut self,
        listener: impl Fn(&MouseUpEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.mouse_up_listener = Some(Box::new(listener));
        self
    }

    fn is_clickable(&self) -> bool {
        self.mouse_down_listener.is_some()
    }
}

impl ParentElement for Container {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl Styled for Container {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Container {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let is_clickable = self.is_clickable();

        self.base
            .bg(cx.theme().colors().element_background)
            .border()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .overflow_hidden()
            .when(is_clickable, |this| {
                this.hover(|this| this.bg(cx.theme().colors().element_background_hover))
                    .cursor_pointer()
            })
            .when_some(self.mouse_down_listener, |this, mouse_down_listener| {
                this.on_mouse_down(gpui::MouseButton::Left, mouse_down_listener)
            })
            .when_some(self.mouse_up_listener, |this, mouse_up_listener| {
                this.on_mouse_up(gpui::MouseButton::Left, mouse_up_listener)
            })
            .children(self.children)
    }
}
