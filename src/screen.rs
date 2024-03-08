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
    pub screen: Model<Screen>,
    layout: View<LayoutView>,
}

impl ScreenView {
    pub fn build(screen: Model<Screen>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let layout_model = cx.new_model({
                let screen = screen.clone();
                move |cx| screen.read(cx).layout.clone()
            });
            let layout = LayoutView::build(layout_model, cx);

            cx.observe(&screen, |this: &mut Self, screen, cx| {
                this.layout.update(cx, |layout, cx| {
                    layout.layout.update(cx, |layout, cx| {
                        *layout = screen.read(cx).layout.clone();
                    })
                });
                cx.notify();
            })
            .detach();

            let this = Self { screen, layout };

            this
        })
    }
}

impl Render for ScreenView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
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
            .child(self.layout.clone())
            .child(status_bar)
    }
}
