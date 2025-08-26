use gpui::prelude::*;
use gpui::{AnyElement, App, Div, StyleRefinement, Window, div};
use smallvec::SmallVec;

use crate::org::h_divider;
use crate::theme::{ActiveTheme, InteractiveColor};

pub fn section(title: &'static str) -> Section {
    Section::new(title)
}

#[derive(IntoElement)]
pub struct Section {
    title: &'static str,
    children: SmallVec<[AnyElement; 2]>,
    base: Div,
}

impl Section {
    fn new(title: &'static str) -> Self {
        Self { title, children: SmallVec::new(), base: div() }
    }
}

impl ParentElement for Section {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Section {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Section {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let header = div()
            .flex()
            .items_center()
            .gap_2()
            .text_color(cx.theme().colors.text.muted())
            .child(self.title)
            .child(h_divider(cx));

        self.base.child(header).children(self.children)
    }
}
