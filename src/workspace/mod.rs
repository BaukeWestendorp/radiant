use std::fmt::Display;

use gpui::{
    div, white, AppContext, FocusHandle, FocusableView, InteractiveElement, IntoElement, Model,
    ParentElement, Render, Styled, View, ViewContext,
};

use crate::show::Show;
use screen::Screen;

// pub mod layout;
pub mod screen;

pub mod cmd {
    use gpui::actions;

    actions!(show_cmd, [Store, Clear]);
}

pub struct Workspace {
    pub screen: View<Screen>,

    focus_handle: FocusHandle,

    programmer_state: ProgrammerState,
}

impl Workspace {
    pub fn new(show: Model<Show>, cx: &mut ViewContext<Self>) -> Self {
        cx.observe(&show, |_, _, cx| cx.notify()).detach();

        let screen = Screen::build(show, cx);

        let focus_handle = cx.focus_handle();

        Self {
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

    fn cmd_store(&mut self, _action: &cmd::Store, cx: &mut ViewContext<Self>) {
        self.set_programmer_state(ProgrammerState::Store, cx);
    }

    fn cmd_clear(&mut self, _action: &cmd::Clear, cx: &mut ViewContext<Self>) {
        self.set_programmer_state(ProgrammerState::Normal, cx);
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .key_context("Workspace")
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
