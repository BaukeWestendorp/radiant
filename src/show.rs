use std::{collections::HashMap, fmt::Display};

use gpui::{
    div, rgb, white, AppContext, Context, FocusHandle, FocusableView, Global, InteractiveElement,
    IntoElement, Model, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use crate::presets::Presets;

use super::screen::{Screen, ScreenView};

pub mod cmd {
    use gpui::actions;

    actions!(show_cmd, [Store, Clear]);
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Show {
    pub presets: Presets,
    pub programmer_state: ProgrammerState,
    screens: HashMap<ScreenId, Screen>,
}

impl Show {
    pub fn new() -> Self {
        Self {
            presets: Presets::new(),
            programmer_state: ProgrammerState::default(),
            screens: HashMap::new(),
        }
    }

    pub fn global(cx: &AppContext) -> &Self {
        cx.global()
    }

    pub fn update<V: 'static>(
        cx: &mut ViewContext<V>,
        f: impl FnOnce(&mut Show, &mut ViewContext<V>),
    ) {
        cx.update_global::<Show, _>(f);
    }

    pub fn add_screen(&mut self, screen: Screen) -> ScreenId {
        let id = self.get_new_screen_id();
        self.screens.insert(id.clone(), screen);
        id
    }

    pub fn get_screen(&self, id: &ScreenId) -> Option<&Screen> {
        self.screens.get(id)
    }

    pub fn get_screen_mut(&mut self, id: &ScreenId) -> Option<&mut Screen> {
        self.screens.get_mut(id)
    }

    pub fn screens(&self) -> &HashMap<ScreenId, Screen> {
        &self.screens
    }

    fn get_new_screen_id(&self) -> ScreenId {
        // TODO: This is not a good way to get a new id. This only works if you can't remove colors.
        ScreenId(self.screens.len())
    }
}

impl Global for Show {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ScreenId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
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

pub struct ShowView {
    screens: Vec<Model<Screen>>,
    programmer_state: ProgrammerState,

    focus_handle: FocusHandle,
}

impl FocusableView for ShowView {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ShowView {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let screens = Self::get_screens(cx);

            let focus_handle = cx.focus_handle();

            let this = Self {
                screens,
                programmer_state: Show::global(cx).programmer_state,
                focus_handle,
            };

            cx.observe_global::<Show>(move |this: &mut Self, cx| {
                this.screens = Self::get_screens(cx);
                this.programmer_state = Show::global(cx).programmer_state;
                cx.notify();
            })
            .detach();

            this
        })
    }

    fn get_screens(cx: &mut AppContext) -> Vec<Model<Screen>> {
        Show::global(cx)
            .screens()
            .clone()
            .into_iter()
            .map(|(_id, screen)| cx.new_model(|_cx| screen))
            .collect()
    }

    fn cmd_store(&mut self, _action: &cmd::Store, cx: &mut ViewContext<Self>) {
        Show::update(cx, |show, cx| {
            show.programmer_state = ProgrammerState::Store;
            cx.notify();
        });
    }

    fn cmd_clear(&mut self, _action: &cmd::Clear, cx: &mut ViewContext<Self>) {
        Show::update(cx, |show, cx| {
            show.programmer_state = ProgrammerState::Normal;
            cx.notify();
        });
    }
}

impl Render for ShowView {
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
            .track_focus(&self.focus_handle)
            .key_context("Show")
            .on_action(cx.listener(Self::cmd_store))
            .on_action(cx.listener(Self::cmd_clear))
            .font("Zed Sans")
            .text_color(white())
            .children({
                self.screens
                    .iter()
                    .map(|screen| ScreenView::build(screen.clone(), cx))
            })
            .child(status_bar)
    }
}
