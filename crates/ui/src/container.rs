use gpui::{
    div, AnyElement, AppContext, Div, Hsla, IntoElement, MouseDownEvent, MouseUpEvent,
    ParentElement, RenderOnce, StyleRefinement, Styled, WindowContext,
};
use smallvec::SmallVec;
use theme::ActiveTheme;

pub struct ContainerStyle {
    pub background: Hsla,
    pub border: Hsla,
}

#[allow(clippy::type_complexity)]
#[derive(IntoElement)]
pub struct Container {
    base: Div,
    children: SmallVec<[AnyElement; 2]>,
    mouse_down_listener: Option<Box<dyn Fn(&MouseDownEvent, &mut WindowContext) + 'static>>,
    mouse_up_listener: Option<Box<dyn Fn(&MouseUpEvent, &mut WindowContext) + 'static>>,
    style: ContainerStyle,
}

impl Container {
    pub fn new(cx: &AppContext) -> Self {
        Self {
            base: div(),
            children: SmallVec::new(),
            mouse_down_listener: None,
            mouse_up_listener: None,
            style: ContainerStyle {
                background: cx.theme().colors().element_background,
                border: cx.theme().colors().border,
            },
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

    pub fn container_style(mut self, style: ContainerStyle) -> Self {
        self.style = style;
        self
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
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        self.base
            .bg(self.style.background)
            .border()
            .border_color(self.style.border)
            .rounded_md()
            .overflow_hidden()
            .children(self.children)
    }
}
