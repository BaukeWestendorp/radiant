use std::fmt::Display;

use gpui::{
    div, white, AppContext, Context, FocusHandle, FocusableView, Global, InteractiveElement,
    IntoElement, Model, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use crate::{
    dmx::color::DmxColor,
    layout::Layout,
    presets::{ColorPreset, Presets},
    window::{Window, WindowKind},
};

use super::screen::{Screen, ScreenView};

pub mod cmd {
    use gpui::actions;

    actions!(show_cmd, [Store, Clear, Test]);
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Show {
    pub presets: Presets,
    pub programmer_state: ProgrammerState,
    screen: Screen,
}

impl Show {
    pub fn new() -> Self {
        Self {
            presets: Presets::new(),
            programmer_state: ProgrammerState::default(),
            screen: Screen {
                layout: Layout {
                    windows: vec![Window {
                        kind: WindowKind::ColorPresetPool,
                        cols: 3,
                        rows: 3,
                    }],
                },
            },
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
    screen: Model<Screen>,
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
            let focus_handle = cx.focus_handle();

            let this = Self {
                screen: cx.new_model(|cx| Show::global(cx).screen.clone()),
                programmer_state: Show::global(cx).programmer_state,
                focus_handle,
            };

            cx.observe_global::<Show>(move |this: &mut Self, cx| {
                this.screen.update(cx, |screen, cx| {
                    *screen = Show::global(cx).screen.clone();
                });
                this.programmer_state = Show::global(cx).programmer_state;
                cx.notify();
            })
            .detach();

            this
        })
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

    fn cmd_test(&mut self, _action: &cmd::Test, cx: &mut ViewContext<Self>) {
        Show::update(cx, |show, _cx| {
            show.presets
                .add_color_preset(ColorPreset::new("Magneta", DmxColor::new(255, 0, 255)));
        });
    }
}

impl Render for ShowView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let screen = ScreenView::build(self.screen.clone(), cx);

        div()
            .track_focus(&self.focus_handle)
            .key_context("Show")
            .on_action(cx.listener(Self::cmd_store))
            .on_action(cx.listener(Self::cmd_clear))
            .on_action(cx.listener(Self::cmd_test))
            .font("Zed Sans")
            .text_color(white())
            .size_full()
            .child(screen)
    }
}
