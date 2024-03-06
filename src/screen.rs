use gpui::{
    div, Context, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use super::layout::{Layout, LayoutView};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Screen {
    pub layout: Layout,
}

pub struct ScreenView {
    layout: Model<Layout>,
}

impl ScreenView {
    pub fn build(screen: Model<Screen>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let layout = cx.new_model({
                let screen = screen.clone();
                move |cx| screen.read(cx).layout.clone()
            });

            let this = Self { layout };

            cx.observe(&screen, move |this: &mut Self, screen, cx| {
                this.layout.update(cx, {
                    let screen = screen.clone();
                    move |layout, cx| {
                        *layout = screen.read(cx).layout.clone();
                        cx.notify();
                    }
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
