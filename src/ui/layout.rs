use gpui::{
    div, Context, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::show::{Layout, Window};

use super::window::WindowView;

pub struct LayoutView {
    windows: Vec<Model<Window>>,
}

impl LayoutView {
    pub fn build(layout: Model<Layout>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let this = Self {
                windows: layout
                    .read(cx)
                    .windows
                    .clone()
                    .iter()
                    .map(|window| cx.new_model(|_cx| window.clone()))
                    .collect(),
            };

            cx.observe(&layout, |this: &mut Self, layout, cx| {
                let layout = layout.read(cx);
                this.windows = layout
                    .windows
                    .clone()
                    .iter()
                    .map(|window| cx.new_model(|_cx| window.clone()))
                    .collect();
                cx.notify();
            })
            .detach();

            this
        })
    }
}

impl Render for LayoutView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let window_views = self
            .windows
            .iter()
            .map(|window_model| WindowView::build(window_model.clone(), cx))
            .collect::<Vec<_>>();

        div().size_full().children(window_views)
    }
}
