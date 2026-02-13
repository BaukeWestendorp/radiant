use std::collections::HashMap;

use gpui::{App, Entity, SharedString, Window};
use rui::PoolTileDelegate;

use crate::{
    app::state::AppState,
    object::{Effect, EffectId},
};

pub struct EffectsPoolTile {}

impl EffectsPoolTile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn effects(&self, cx: &App) -> Entity<HashMap<EffectId, Effect>> {
        AppState::show(cx).effects()
    }

    pub fn effect<'a>(&self, slot_id: u32, cx: &'a App) -> Option<&'a Effect> {
        self.effects(cx).read(cx).get(&slot_id)
    }
}

impl PoolTileDelegate for EffectsPoolTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Effects".into()
    }

    fn is_occupied(&self, slot_id: u32, cx: &App) -> bool {
        self.effect(slot_id, cx).is_some()
    }

    fn occupied_label(&self, slot_id: u32, cx: &App) -> String {
        self.effect(slot_id, cx).map(|g| g.name()).unwrap_or("<unknown>").to_string()
    }

    fn on_activate_slot(&mut self, slot_id: u32, _window: &mut Window, cx: &mut App) {
        todo!();
    }
}
