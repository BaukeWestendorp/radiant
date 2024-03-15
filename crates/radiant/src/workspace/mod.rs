use std::fmt::Display;

use gpui::{
    div, white, AppContext, FocusHandle, FocusableView, InteractiveElement, IntoElement, Model,
    ParentElement, Render, Styled, View, ViewContext,
};

use crate::show::Show;
use screen::Screen;
pub mod layout;
pub mod screen;

pub mod actions {
    use gpui::actions;

    actions!(workspace_actions, [OpenShow, SaveShow]);

    pub mod cmd {
        use gpui::actions;

        actions!(workspace, [Store, Clear]);
    }
}

pub struct Workspace {
    show: Model<Show>,

    pub screen: View<Screen>,

    focus_handle: FocusHandle,

    programmer_state: ProgrammerState,
}

impl Workspace {
    const SHOW_FILE_PATH: &'static str = "show.json";

    pub fn new(show: Model<Show>, cx: &mut ViewContext<Self>) -> Self {
        cx.observe(&show, |_, _, cx| cx.notify()).detach();

        let screen = Screen::build(show.clone(), cx);

        let focus_handle = cx.focus_handle();

        Self {
            show,
            screen,
            focus_handle,
            programmer_state: ProgrammerState::default(),
        }
    }

    pub fn programmer_state(&self) -> ProgrammerState {
        self.programmer_state
    }

    pub fn set_programmer_state(&mut self, state: ProgrammerState, cx: &mut ViewContext<Self>) {
        self.programmer_state = state;
        cx.notify();
    }

    pub fn open_show(&mut self, _action: &actions::OpenShow, cx: &mut ViewContext<Self>) {
        self.show.update(cx, |show, cx| {
            match Show::from_file(Self::SHOW_FILE_PATH) {
                Ok(loaded_show) => *show = loaded_show,
                Err(e) => {
                    log::error!("Failed to open show: {}", e);
                }
            }

            cx.notify();
            log::info!("Opened show file '{}'", Self::SHOW_FILE_PATH);
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

    pub fn cmd_store(&mut self, _action: &actions::cmd::Store, cx: &mut ViewContext<Self>) {
        self.set_programmer_state(ProgrammerState::Store, cx);
    }

    pub fn cmd_clear(&mut self, _action: &actions::cmd::Clear, cx: &mut ViewContext<Self>) {
        self.set_programmer_state(ProgrammerState::Normal, cx);
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .key_context("Workspace")
            .on_action(cx.listener(Self::open_show))
            .on_action(cx.listener(Self::save_show))
            .on_action(cx.listener(Self::cmd_store))
            .on_action(cx.listener(Self::cmd_clear))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum ProgrammerState {
    #[default]
    Normal,
    Store,
}

impl Display for ProgrammerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "Normal"),
            Self::Store => write!(f, "Store"),
        }
    }
}
