use gpui::prelude::*;
use gpui::{AnyElement, App, FontWeight, SharedString, StyleRefinement, Window, div};
use smallvec::SmallVec;

use crate::{ActiveTheme as _, StyledExt};

pub fn section(title: impl Into<SharedString>) -> Section {
    Section::new(title)
}

#[derive(IntoElement)]
pub struct Section {
    style: StyleRefinement,

    title: SharedString,
    children: SmallVec<[AnyElement; 2]>,
}

impl Section {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self { style: StyleRefinement::default(), title: title.into(), children: SmallVec::new() }
    }
}

impl RenderOnce for Section {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let header = div()
            .w_full()
            .px_1()
            .font_weight(FontWeight::BOLD)
            .border_b_1()
            .border_color(cx.theme().border_secondary)
            .child(self.title);

        let content = div().flex().size_full().refine_style(&self.style).children(self.children);

        div().flex().flex_col().gap_2().child(header).child(content)
    }
}

impl Styled for Section {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Section {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}
