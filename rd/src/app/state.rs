use std::fmt;

use gpui::{App, Entity, Global, Hsla, prelude::*};
use rd_ui::ActiveTheme;

pub(crate) fn init(cx: &mut App) {
    let state = State::new(cx);
    cx.set_global(state);
}

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    mode: Entity<Mode>,
}

impl State {
    pub fn new(cx: &mut App) -> Self {
        Self { mode: cx.new(|_| Mode::default()) }
    }

    pub fn mode(&self) -> Entity<Mode> {
        self.mode.clone()
    }
}

impl Global for State {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Mode {
    #[default]
    Normal,
    Store,
    Rename,
}

impl Mode {
    pub fn color(&self, cx: &App) -> Hsla {
        match self {
            Mode::Normal => cx.theme().fg_primary,
            Mode::Store => cx.theme().indicate.programmer,
            Mode::Rename => cx.theme().indicate.rename,
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Normal => write!(f, "Normal"),
            Mode::Store => write!(f, "Store"),
            Mode::Rename => write!(f, "Rename"),
        }
    }
}
