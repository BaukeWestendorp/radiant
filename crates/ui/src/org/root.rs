use gpui::prelude::*;
use gpui::{AnyElement, App, Div, Interactivity, RenderOnce, StyleRefinement, Window, div, px};
use smallvec::SmallVec;

use crate::interactive::modal::modal_container;
use crate::misc::titlebar;
use crate::theme::ActiveTheme;
use crate::utils::z_stack;

/// The main UI root component. Use [root] to create a new [Root].
#[derive(IntoElement)]
pub struct Root {
    children: SmallVec<[AnyElement; 2]>,
    titlebar_children: SmallVec<[AnyElement; 2]>,
    root: Div,
}

impl Root {
    fn new() -> Self {
        Self { children: SmallVec::new(), titlebar_children: SmallVec::new(), root: div() }
    }

    /// Add a single child element to the titlebar.
    pub fn titlebar_child(mut self, child: impl IntoElement) -> Self {
        self.titlebar_children.extend(std::iter::once(child.into_element().into_any()));
        self
    }

    /// Add multiple child elements to the titlebar.
    pub fn titlebar_children(
        mut self,
        children: impl IntoIterator<Item = impl IntoElement>,
    ) -> Self {
        self.titlebar_children.extend(children.into_iter().map(|child| child.into_any_element()));
        self
    }
}

impl RenderOnce for Root {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .text_color(cx.theme().foreground)
            .font_family("Tamzen")
            .line_height(px(14.0))
            .bg(cx.theme().background)
            .child(titlebar(window, cx).children(self.titlebar_children))
            .child(
                z_stack([
                    div().flex().flex_col().size_full().children(self.children),
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

/// Creates a new [Root] element.
pub fn root() -> Root {
    Root::new()
}
