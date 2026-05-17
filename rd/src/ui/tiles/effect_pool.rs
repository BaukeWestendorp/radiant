use gpui::{App, SharedString, Window};
use rd_core::object::{Effect, Object, ObjectKind, ObjectReference, SlotId};
use rd_ui::PoolTileDelegate;

use crate::state::AppState;

pub struct EffectsPoolTile {}

impl EffectsPoolTile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn effect<'a>(&self, slot_id: u32, cx: &'a App) -> Option<&'a Effect> {
        AppState::engine(cx)
            .objects()
            .get(ObjectReference::Slot(ObjectKind::Effect, SlotId::new(slot_id).unwrap()))
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
        self.effect(slot_id, cx).map(|effect| effect.name()).unwrap_or("<unknown>").to_string()
    }

    fn on_activate_slot(&mut self, _slot_id: u32, _window: &mut Window, _cx: &mut App) {}
}
