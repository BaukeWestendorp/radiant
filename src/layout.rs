use gpui::{
    div, rgb, AppContext, Context, IntoElement, Model, ParentElement, Render, Styled, View,
    ViewContext, VisualContext, WindowContext,
};

use crate::window::Window;

use super::window::WindowView;

pub const LAYOUT_CELL_SIZE: usize = 80;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Layout {
    pub windows: Vec<Window>,
}

pub struct LayoutView {
    windows: Vec<Model<Window>>,
}

impl LayoutView {
    pub fn build(layout: Model<Layout>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let windows = Self::get_windows(&layout, cx);

            let this = Self { windows };

            cx.observe(&layout, |this: &mut Self, layout, cx| {
                this.windows = Self::get_windows(&layout, cx);
                cx.notify();
            })
            .detach();

            this
        })
    }

    fn get_windows(layout: &Model<Layout>, cx: &mut AppContext) -> Vec<Model<Window>> {
        layout
            .read(cx)
            .windows
            .clone()
            .iter()
            .map(|window| cx.new_model(|_cx| window.clone()))
            .collect()
    }
}

impl Render for LayoutView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let window_views = self
            .windows
            .iter()
            .map(|window_model| WindowView::build(window_model.clone(), cx))
            .collect::<Vec<_>>();

        div()
            .size_full()
            .bg(rgb(0x181818))
            .p_2()
            .children(window_views)
    }
}
