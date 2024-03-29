use backstage::show::Show;
use gpui::{
    div, AppContext, FocusHandle, FocusableView, InteractiveElement, IntoElement, Model,
    ParentElement, Render, Styled, View, ViewContext, VisualContext, WindowContext, WindowHandle,
    WindowOptions,
};

use crate::theme::ActiveTheme;
use crate::ui::text_input::{self, TextInput};
use crate::workspace;

use super::window_grid::WindowGridView;
use super::WindowGrid;

pub struct Screen {
    window_grid: View<WindowGridView>,
    command_line: View<CommandLine>,

    focus_handle: FocusHandle,

    show: Model<Show>,
}

impl Screen {
    pub fn open_window(
        show: Model<Show>,
        window_grid: Model<WindowGrid>,
        cx: &mut AppContext,
    ) -> WindowHandle<Self> {
        let window_options = WindowOptions::default();
        cx.open_window(window_options, |cx| {
            cx.new_view(|cx| Self {
                window_grid: WindowGridView::build(window_grid, show.clone(), cx),
                command_line: CommandLine::build(show.clone(), cx),
                focus_handle: cx.focus_handle(),
                show,
            })
        })
    }

    fn cmd_clear(&mut self, command: &workspace::action::Command, cx: &mut ViewContext<Self>) {
        self.show.update(cx, |show, cx| {
            if let Err(err) = show.execute_command(&command.0) {
                log::error!("Failed to execute Clear command: {err}");
            }
            cx.notify();
        })
    }
}

impl FocusableView for Screen {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Screen {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .key_context("Screen")
            .on_action(cx.listener(Self::cmd_clear))
            .font("Zed Sans")
            .text_color(cx.theme().colors().text)
            .bg(cx.theme().colors().background)
            .size_full()
            .flex()
            .flex_col()
            .child(div().size_full().child(self.window_grid.clone()))
            .child(div().h_10().child(self.command_line.clone()))
    }
}

pub struct CommandLine {
    text_input: View<TextInput>,

    focus_handle: FocusHandle,

    show: Model<Show>,
}

impl CommandLine {
    pub fn build(show: Model<Show>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let focus_handle = cx.focus_handle();
            let text_input =
                cx.new_view(|cx| TextInput::new(None, "Command line", focus_handle.clone(), cx));

            cx.subscribe(
                &text_input,
                |cmd_line: &mut CommandLine, _text_input, event, cx| match event {
                    text_input::Event::Submit(input) => {
                        cmd_line.handle_submit_command_input(input, cx)
                    }
                },
            )
            .detach();

            Self {
                text_input,
                focus_handle,
                show,
            }
        })
    }

    fn handle_submit_command_input(&mut self, input: &str, cx: &mut WindowContext) {
        if input.is_empty() {
            return;
        }

        self.text_input.update(cx, |text_input, cx| {
            text_input.clear(cx);
            self.show.update(cx, |show, cx| {
                if let Err(err) = show.execute_command_str(input) {
                    log::error!("Failed to execute command: {}", err.to_string())
                } else {
                    cx.notify();
                }
            })
        })
    }
}

impl FocusableView for CommandLine {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CommandLine {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .border_t()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().element_background)
            .flex()
            .flex_shrink()
            .items_center()
            .px_3()
            .child(self.text_input.clone())
            .track_focus(&self.focus_handle)
    }
}
