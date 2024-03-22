use std::time::Duration;

use gpui::{
    div, white, AppContext, FocusHandle, FocusableView, InteractiveElement, IntoElement, Model,
    ParentElement, Render, Styled, Timer, View, ViewContext,
};

use crate::show::Show;
use screen::Screen;

pub mod layout;
pub mod screen;

pub mod actions {
    use gpui::actions;

    actions!(workspace_actions, [OpenShow, SaveShow]);
}

const DMX_OUTPUT_RATE: Duration = Duration::from_millis(1000 / 40);

pub struct Workspace {
    show: Model<Show>,

    pub screen: View<Screen>,

    focus_handle: FocusHandle,
}

impl Workspace {
    const SHOW_FILE_PATH: &'static str = "show.json";

    pub fn new(show: Model<Show>, cx: &mut ViewContext<Self>) -> Self {
        cx.observe(&show, |_, _, cx| cx.notify()).detach();

        let screen = Screen::build(show.clone(), cx);

        let focus_handle = cx.focus_handle();

        let this = Self {
            show,
            screen,
            focus_handle,
        };

        this.dmx_output_interval(cx);

        this
    }

    pub fn open_show(&mut self, _action: &actions::OpenShow, cx: &mut ViewContext<Self>) {
        self.show.update(cx, |show, cx| {
            match Show::from_file(Self::SHOW_FILE_PATH) {
                Ok(loaded_show) => *show = loaded_show,
                Err(e) => {
                    log::error!("{}", e);
                    return;
                }
            }
            log::info!("Opened show file '{}'", Self::SHOW_FILE_PATH);

            show.init();

            cx.notify();
        });
    }

    pub fn save_show(&mut self, _action: &actions::SaveShow, cx: &mut ViewContext<Self>) {
        match self.show.read(cx).save_to_file(Self::SHOW_FILE_PATH) {
            Ok(_) => {
                log::info!("Saved show file '{}'", Self::SHOW_FILE_PATH);
            }
            Err(e) => {
                log::error!("Failed to save show: {}", e);
            }
        }
    }

    fn dmx_output_interval(&self, cx: &mut ViewContext<Self>) {
        cx.spawn(|this, mut cx| async move {
            Timer::after(DMX_OUTPUT_RATE).await;
            this.update(&mut cx, |this, cx| {
                log::trace!("Outputting DMX data...");
                this.show.update(cx, |show, _cx| {
                    show.update_dmx_output();
                    show.send_output_over_active_protocols();
                });
                this.dmx_output_interval(cx);
            })
            .unwrap();
        })
        .detach();
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .key_context("Workspace")
            .on_action(cx.listener(Self::open_show))
            .on_action(cx.listener(Self::save_show))
            .font("Zed Sans")
            .text_color(white())
            .size_full()
            .child(self.screen.clone())
    }
}

impl FocusableView for Workspace {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
