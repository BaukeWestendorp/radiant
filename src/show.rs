use gpui::{
    div, AppContext, Context, FocusHandle, FocusableView, Global, InteractiveElement, IntoElement,
    Model, ParentElement, Render, View, ViewContext, VisualContext, WindowContext,
};
use serde::{Deserialize, Serialize};

use crate::{presets::Presets, ui::screen::Screen};

pub mod cmd {
    use gpui::actions;

    actions!(cmd, [Store, Clear]);
}

#[derive(Clone)]
pub struct Show {
    pub presets: Presets,
    pub programmer_state: ProgrammerState,
}

#[derive(Clone)]
pub struct ShowModel {
    pub inner: Model<Show>,
}

impl ShowModel {
    pub fn init(cx: &mut WindowContext) {
        let inner = cx.new_model(|_cx| Show {
            presets: Presets::new(),
            programmer_state: ProgrammerState::default(),
        });

        let this = ShowModel { inner };

        cx.set_global(this);
    }

    pub fn update(cx: &mut WindowContext, f: impl FnOnce(&mut Self, &mut WindowContext)) {
        cx.update_global::<ShowModel, _>(|mut this, cx| f(&mut this, cx));
    }

    pub fn global(cx: &AppContext) -> &Self {
        cx.global()
    }
}

impl Global for ShowModel {}

#[derive(Clone)]
pub struct ShowView {
    screen: View<Screen>,
    focus_handle: FocusHandle,
}

impl ShowView {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let focus_handle = cx.focus_handle();

            let screen = Screen::build(cx);

            Self {
                screen,
                focus_handle,
            }
        })
    }

    pub fn store_command(&mut self, _action: &cmd::Store, cx: &mut ViewContext<Self>) {
        self.update_programmer_state(ProgrammerState::Store, cx)
    }

    pub fn clear_command(&mut self, _action: &cmd::Clear, cx: &mut ViewContext<Self>) {
        self.update_programmer_state(ProgrammerState::Normal, cx)
    }

    fn update_programmer_state(
        &mut self,
        programmer_state: ProgrammerState,
        cx: &mut ViewContext<Self>,
    ) {
        ShowModel::update(cx, |model, cx| {
            model.inner.update(cx, |show, cx| {
                show.programmer_state = programmer_state;
                cx.notify();
            })
        });
    }
}

impl FocusableView for ShowView {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ShowView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .key_context("Show")
            .on_action(cx.listener(Self::store_command))
            .on_action(cx.listener(Self::clear_command))
            .child(self.screen.clone())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ProgrammerState {
    #[default]
    Normal,
    Store,
}

impl ToString for ProgrammerState {
    fn to_string(&self) -> String {
        match self {
            ProgrammerState::Normal => "Normal".to_string(),
            ProgrammerState::Store => "Store".to_string(),
        }
    }
}
