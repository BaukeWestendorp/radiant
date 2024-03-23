use gpui::{
    div, rgb, IntoElement, Model, ParentElement, Render, SharedString, Styled, View, ViewContext,
    VisualContext,
};

use crate::show::Show;
use crate::workspace::Workspace;

use super::layout::Layout;

pub struct Screen {
    pub layout: View<Layout>,
    command_line: SharedString,
}

impl Screen {
    pub fn build(show: Model<Show>, cx: &mut ViewContext<Workspace>) -> View<Self> {
        cx.new_view(|cx| {
            let layout = Layout::build(show.clone(), cx);

            cx.observe(&show, |this, show, cx| {
                this.command_line = show.read(cx).command_line.clone();
                cx.notify();
            })
            .detach();

            Self {
                layout,
                command_line: "".into(),
            }
        })
    }
}

impl Render for Screen {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let content = self.layout.clone();

        let command_list = div()
            .flex()
            .gap_2()
            .child("> ")
            .child(self.command_line.clone());

        let status_bar = div()
            .h_10()
            .px_2()
            .border_t()
            .border_color(rgb(0x3a3a3a))
            .flex()
            .items_center()
            .bg(rgb(0x2a2a2a))
            .child(command_list);

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(content)
            .child(status_bar)
    }
}
