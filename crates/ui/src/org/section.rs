use crate::{ActiveTheme, InteractiveColor};
use gpui::{AnyElement, App, Div, StyleRefinement, Window, div, prelude::*};
use smallvec::SmallVec;

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
    fn render(self, _w: &mut Window, cx: &mut App) -> impl IntoElement {
        let header = div()
            .w_full()
            .flex()
            .items_center()
            .gap_2()
            .text_color(cx.theme().text_primary.muted())
            .child(self.title)
            .child(crate::divider(cx));

        self.base.child(header).children(self.children)
    }
}
