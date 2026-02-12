use anyhow::Result;
use gpui::{App, Entity, Global, ReadGlobal as _, prelude::*};
use zeevonk::Zeevonk;

use crate::{effect_engine::EffectEngine, show::Show, showfile::Showfile};

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
    effect_engine: Entity<EffectEngine>,
}

impl AppState {
    pub fn new(showfile: Showfile, cx: &mut App) -> Result<Self> {
        let zeevonk = Zeevonk::new(showfile.zv_project_file().clone())?;
        zeevonk.start();

        let show = Show::from_showfile(showfile, cx);
        let effect_engine = cx.new(|cx| EffectEngine::new(show.effects(), cx));

        // Set highlighed values in Zeevonk if the selection changes.
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

        // Update highlighted fixtures in Zeevonk when highlight mode changes.
        cx.observe(&show.modes(), |modes, cx| {
            if modes.read(cx).highlight {
                let selection = AppState::show(cx).selection().read(cx);
                AppState::zeevonk(cx).set_highlighted_fixtures(selection);
            } else {
                AppState::zeevonk(cx).clear_highlighted_fixtures();
            }
        })
        .detach();

        Ok(Self { zeevonk, show, effect_engine })
    }

    pub fn zeevonk(cx: &App) -> &Zeevonk {
        &Self::global(cx).zeevonk
    }

    pub fn show(cx: &App) -> &Show {
        &Self::global(cx).show
    }

    pub fn effect_engine(cx: &App) -> Entity<EffectEngine> {
        Self::global(cx).effect_engine.clone()
    }
}

impl Global for AppState {}
