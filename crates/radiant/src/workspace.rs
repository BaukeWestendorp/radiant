use anyhow::Result;
use backstage::show::Show;
use backstage::showfile::Showfile;
use gdtf_share::GdtfShare;
use gpui::{
    div, AppContext, Context, FocusHandle, FocusableView, InteractiveElement, IntoElement, Model,
    ParentElement, Render, Styled, Task, View, ViewContext, VisualContext, WindowContext,
    WindowHandle, WindowOptions,
};

use std::env;
use std::fs::File;

pub mod action {
    use gpui::actions;

    actions!(workspace, [Debug]);
}

use crate::ui::text_input::{self, TextInput};

pub struct Workspace {
    command_line: View<CommandLine>,

    focus_handle: FocusHandle,

    show: Model<Show>,
}

impl Workspace {
    pub fn new(cx: &mut AppContext) -> Task<Result<WindowHandle<Self>>> {
        let window_options = WindowOptions::default();

        cx.spawn(move |mut cx| async move {
            let show = get_show().await?;
            let show_model = cx.new_model(|_cx| show)?;

            cx.open_window(window_options, |cx| {
                cx.new_view(|cx| Self {
                    command_line: CommandLine::build(show_model.clone(), cx),
                    focus_handle: cx.focus_handle(),
                    show: show_model,
                })
            })
        })
    }

    fn debug(&mut self, _: &action::Debug, cx: &mut ViewContext<Self>) {
        self.show.update(cx, |show, _cx| {
            let stage_output = show.get_stage_output();
            log::debug!("{:?}", stage_output);
        })
    }
}

async fn get_show() -> Result<Show> {
    let file = File::open("show.json")?;
    let showfile = Showfile::from_file(file)?;
    let user = env::var("GDTF_SHARE_API_USER")?;
    let password = env::var("GDTF_SHARE_API_PASSWORD")?;
    let gdtf_share = GdtfShare::auth(user, password).await?;
    Ok(Show::new(showfile, gdtf_share).await)
}

impl FocusableView for Workspace {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .key_context("Workspace")
            .font("Zed Sans")
            .on_action(cx.listener(Self::debug))
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

    show: Model<Show>,
}

impl CommandLine {
    pub fn build(show: Model<Show>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let text_input = cx.new_view(|cx| TextInput::new(None, "Command line", cx));

            cx.subscribe(
                &text_input,
                |cmd_line: &mut CommandLine, _text_input, event, cx| match event {
                    text_input::Event::Submit(input) => {
                        cmd_line.handle_submit_command_input(input, cx)
                    }
                },
            )
            .detach();

            Self { text_input, show }
        })
    }

    fn handle_submit_command_input(&mut self, input: &str, cx: &mut WindowContext) {
        if input.is_empty() {
            return;
        }

        self.text_input.update(cx, |text_input, cx| {
            text_input.clear(cx);
            self.show.update(cx, |show, _cx| {
                if let Err(err) = show.execute_command_str(input) {
                    log::error!("Failed to execute command: {}", err.to_string())
                }
            })
        })
    }
}

impl Render for CommandLine {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
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
