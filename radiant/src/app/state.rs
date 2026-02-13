use std::sync::Arc;

use anyhow::Result;
use gpui::{App, Entity, Global, ReadGlobal as _, prelude::*};
use zeevonk::Zeevonk;

use crate::{effect::engine::EffectEngine, show::Show, showfile::Showfile};

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
    zeevonk: Arc<Zeevonk>,

    show: Show,

    effect_engine: Entity<EffectEngine>,
}

impl AppState {
    pub fn new(showfile: Showfile, cx: &mut App) -> Result<Self> {
        let zeevonk = Arc::new(Zeevonk::new(showfile.zv_project_file().clone())?);
        zeevonk.start();

        let show = Show::from_showfile(showfile, cx);
        let effect_engine = cx.new({
            let zeevonk = Arc::clone(&zeevonk);
            let effects = show.effects().clone();
            move |cx| EffectEngine::new(effects, zeevonk, cx)
        });

        // FIXME: This is just for testing. Get from recipe or something.
        let effect_ids = show.effects().read(cx).keys().cloned().collect::<Vec<_>>();
        cx.defer({
            let effect_engine = effect_engine.clone();
            move |cx| {
                for effect_id in effect_ids {
                    let group_id = 9;
                    effect_engine.update(cx, |effect_engine, cx| {
                        effect_engine.start_effect(effect_id, group_id, cx).unwrap();
                    });
                }
            }
        });

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

    pub fn zeevonk(cx: &App) -> &Arc<Zeevonk> {
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
