use gpui::{
    div, AnyElement, AppContext, Div, Hsla, InteractiveElement, Interactivity, IntoElement,
    ParentElement, RenderOnce, StyleRefinement, Styled, WindowContext,
};
use smallvec::SmallVec;
use theme::ActiveTheme;

pub struct ContainerStyle {
    pub background: Hsla,
    pub border: Hsla,
}

#[derive(IntoElement)]
pub struct Container {
    base: Div,
    children: SmallVec<[AnyElement; 2]>,
    interactivity: Interactivity,
    style: ContainerStyle,
}

impl Container {
    pub fn new(cx: &AppContext) -> Self {
        Self {
            base: div(),
            children: SmallVec::new(),
            interactivity: Interactivity::default(),
            style: ContainerStyle {
                background: cx.theme().colors().element_background,
                border: cx.theme().colors().border,
            },
        }
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

impl InteractiveElement for Container {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
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
