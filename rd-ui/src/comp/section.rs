use gpui::{
    AnyElement, App, RenderOnce, SharedString, StyleRefinement, Styled, Window, div, prelude::*,
};

use crate::{ActiveTheme, StyledExt, v_flex};

#[derive(IntoElement)]
pub struct Section {
    title: SharedString,
    content: AnyElement,
    style: StyleRefinement,
}

impl Section {
    pub fn new(title: impl Into<SharedString>, content: impl IntoElement) -> Self {
        Self {
            title: title.into(),
            content: content.into_any_element(),
            style: StyleRefinement::default(),
        }
    }
}

impl RenderOnce for Section {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let header = div()
            .w_full()
            .border_b_1()
            .border_color(cx.theme().border_primary)
            .child(div().font_bold().child(self.title));

        let content = div().p_2().child(self.content);

        v_flex().refine_style(&self.style).child(header).child(content)
    }
}

impl Styled for Section {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
