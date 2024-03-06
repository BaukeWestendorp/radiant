use gpui::{
    div, Context, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::show::{Layout, Screen};

use super::layout::LayoutView;

pub struct ScreenView {
    layout: Model<Layout>,
}

impl ScreenView {
    pub fn build(screen: Model<Screen>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let this = Self {
                layout: cx.new_model({
                    let screen = screen.clone();
                    move |cx| screen.read(cx).layout.clone()
                }),
            };

            cx.observe(&screen, move |this: &mut Self, screen, cx| {
                let screen = screen.clone();
                this.layout.update(cx, move |layout, cx| {
                    let screen = screen.read(cx);
                    *layout = screen.layout.clone();
                    cx.notify();
                });
            })
            .detach();

            this
        })
    }
}

impl Render for ScreenView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let layout_view = LayoutView::build(self.layout.clone(), cx);

        div()
            .border()
            .border_color(gpui::green())
            .child(layout_view)
    }
}
