use gpui::{
    div, rgb, Context, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::show::Show;

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

        let show = Show::global(cx);

        let status_bar = div()
            .child(format!("Programmer State: {}", show.programmer_state))
            .h_10()
            .px_2()
            .border_t()
            .border_color(rgb(0x3a3a3a))
            .flex()
            .items_center()
            .bg(rgb(0x2a2a2a));

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(layout_view)
            .child(status_bar)
    }
}
