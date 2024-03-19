use gpui::{
    div, rgb, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext, VisualContext,
};

use crate::cmd::CommandList;
use crate::show::Show;
use crate::workspace::Workspace;

use super::layout::Layout;

pub struct Screen {
    pub layout: View<Layout>,
    command_list: CommandList,
}

impl Screen {
    pub fn build(show: Model<Show>, cx: &mut ViewContext<Workspace>) -> View<Self> {
        cx.new_view(|cx| {
            let layout = Layout::build(show, cx);

            cx.observe_global::<CommandList>(|this: &mut Self, cx| {
                this.command_list = CommandList::global(cx).clone();
                cx.notify();
            })
            .detach();

            Self {
                layout,
                command_list: CommandList::default(),
            }
        })
    }
}

impl Render for Screen {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let content = self.layout.clone();

        let command_list = div().flex().gap_2().child("> ").children(
            CommandList::commands(cx)
                .iter()
                .map(|command| div().child(format!("{}", command))),
        );

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
