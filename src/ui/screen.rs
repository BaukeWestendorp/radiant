use gpui::{div, rgb, IntoElement, ParentElement, Render, Styled, ViewContext, VisualContext};
use serde::{Deserialize, Serialize};

use crate::show::Show;

use super::layout::Layout;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Screen {
    layout: Layout,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            layout: Layout::new(),
        }
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn layout_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }
}

impl Render for Screen {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let show = cx.global::<Show>();

        let status_bar = div()
            .child(format!(
                "Programmer State: {}",
                show.programmer_state().to_string()
            ))
            .flex()
            .bg(rgb(0x303030))
            .text_xs()
            .justify_center()
            .p_1()
            .h_10();

        div()
            .child(cx.new_view(|_| self.layout.clone()))
            .child(status_bar)
            .flex()
            .flex_col()
            .size_full()
    }
}
