use gpui::prelude::*;
use gpui::{
    AnyElement, App, Div, FontWeight, Interactivity, RenderOnce, StyleRefinement, Window, div, px,
};
use smallvec::SmallVec;

use crate::interactive::modal::modal_container;
use crate::theme::ActiveTheme;
use crate::utils::z_stack;

#[derive(IntoElement)]
pub struct Root {
    children: SmallVec<[AnyElement; 2]>,
    root: Div,
}

impl Root {
    pub fn new() -> Self {
        Self { children: SmallVec::new(), root: div() }
    }
}

impl RenderOnce for Root {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .size_full()
            .text_color(cx.theme().colors.text)
            .font_family("Tamzen")
            .font_weight(FontWeight::BOLD)
            .line_height(px(14.0))
            .bg(cx.theme().colors.bg_primary)
            .child(
                z_stack([
                    div().size_full().children(self.children),
                    div().size_full().child(modal_container(cx)),
                ])
                .size_full(),
            )
    }
}

impl Styled for Root {
    fn style(&mut self) -> &mut StyleRefinement {
        self.root.style()
    }
}

impl InteractiveElement for Root {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.root.interactivity()
    }
}

impl ParentElement for Root {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

pub fn root() -> Root {
    Root::new()
}
