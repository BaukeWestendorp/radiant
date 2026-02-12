use anyhow::Result;
use gpui::{App, Global, ReadGlobal as _};
use zeevonk::Zeevonk;

use crate::{show::Show, showfile::Showfile};

pub(crate) fn init(showfile: Showfile, cx: &mut App) -> Result<()> {
    let app_state = AppState::new(showfile, cx)?;
    cx.set_global(app_state);
    Ok(())
}

pub mod action {
    use gpui::{App, KeyBinding, actions};

    use crate::app::state::AppState;

    actions!([ToggleHighlight]);

    pub(crate) fn init(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("h", ToggleHighlight, None)]);

        cx.on_action::<ToggleHighlight>(|_, cx| {
            AppState::show(cx).modes().update(cx, |modes, cx| {
                modes.highlight = !modes.highlight;
                cx.notify();
            });
        });
    }
}

pub struct AppState {
    zeevonk: Zeevonk,

    show: Show,
}

impl AppState {
    pub fn new(showfile: Showfile, cx: &mut App) -> Result<Self> {
        let zeevonk = Zeevonk::new(showfile.zv_project_file().clone())?;
        zeevonk.start();

        let show = Show::from_showfile(&showfile, cx);

        cx.observe(&show.selection(), |selection, cx| {
            let highlight = AppState::show(cx).modes().read(cx).highlight;
            if highlight {
                let selection = selection.read(cx);
                AppState::zeevonk(cx).set_highlighted_fixtures(selection);
            } else {
                AppState::zeevonk(cx).clear_highlighted_fixtures();
            }
        })
        .detach();

        cx.observe(&show.modes(), |modes, cx| {
            if modes.read(cx).highlight {
                let selection = AppState::show(cx).selection().read(cx);
                AppState::zeevonk(cx).set_highlighted_fixtures(selection);
            } else {
                AppState::zeevonk(cx).clear_highlighted_fixtures();
            }
        })
        .detach();

        Ok(Self { zeevonk, show })
    }

    pub fn zeevonk(cx: &App) -> &Zeevonk {
        &Self::global(cx).zeevonk
    }

    pub fn show(cx: &App) -> &Show {
        &Self::global(cx).show
    }
}

impl Global for AppState {}
