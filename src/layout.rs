use gpui::{
    div, rgb, Context, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::window::Window;

use super::window::WindowView;

pub const LAYOUT_CELL_SIZE: usize = 80;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Layout {
    pub windows: Vec<Window>,
}

pub struct LayoutView {
    pub layout: Model<Layout>,
    windows: Vec<View<WindowView>>,
}

impl LayoutView {
    pub fn build(layout: Model<Layout>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let window_models = layout
                .read(cx)
                .windows
                .clone()
                .iter()
                .map(|window| cx.new_model(|_cx| window.clone()))
                .collect::<Vec<_>>();

            let windows = window_models
                .iter()
                .map(|window_model| WindowView::build(window_model.clone(), cx))
                .collect::<Vec<_>>();

            cx.observe(&layout, |this: &mut Self, layout, cx| {
                for (ix, window) in this.windows.iter_mut().enumerate() {
                    window.update(cx, |window, cx| {
                        window.window.update(cx, |window, cx| {
                            *window = layout.read(cx).windows[ix].clone();
                        })
                    })
                }
                cx.notify();
            })
            .detach();

            Self { layout, windows }
        })
    }
}

impl Render for LayoutView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x181818))
            .p_2()
            .children(self.windows.clone())
    }
}
