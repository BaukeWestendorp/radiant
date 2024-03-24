use gpui::{
    div, AppContext, FocusHandle, FocusableView, InteractiveElement, IntoElement, ParentElement,
    Render, Styled, View, ViewContext, VisualContext, WindowContext, WindowHandle, WindowOptions,
};

use crate::ui::editor::TextInput;

pub struct Workspace {
    command_line: View<CommandLine>,

    focus_handle: FocusHandle,
}

impl Workspace {
    pub fn open_window(cx: &mut AppContext) -> WindowHandle<Self> {
        let window_options = WindowOptions::default();

        cx.open_window(window_options, |cx| {
            cx.new_view(|cx| Self {
                command_line: CommandLine::build(cx),
                focus_handle: cx.focus_handle(),
            })
        })
    }
}

impl FocusableView for Workspace {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Workspace {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .key_context("Workspace")
            .font("Zed Sans")
            .text_color(gpui::white())
            .size_full()
            .flex()
            .flex_col()
            .child(div().size_full())
            .child(div().h_10().w_full().child(self.command_line.clone()))
    }
}

pub struct CommandLine {
    text_input: View<TextInput>,
}

impl CommandLine {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            text_input: cx.new_view(|cx| TextInput::new(None, "Command line", cx)),
        })
    }
}

impl Render for CommandLine {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .border_t()
            .border_color(gpui::white())
            .p_3()
            .flex()
            .items_center()
            .child(self.text_input.clone())
    }
}
