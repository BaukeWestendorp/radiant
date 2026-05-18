use gpui::{App, IntoElement, ReadGlobal, SharedString, Window, prelude::*};
use rd_ui::{PoolTileDelegate, h_flex};

use crate::engine::{Effect, Engine, Object as _, ObjectKind, ObjectReference, SlotId};

pub struct EffectPoolTile {}

impl EffectPoolTile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn effect<'a>(&self, slot_id: u32, cx: &'a App) -> Option<&'a Effect> {
        Engine::global(cx)
            .objects()
            .get(ObjectReference::Slot(ObjectKind::Effect, SlotId::new(slot_id).unwrap()))
    }
}

impl PoolTileDelegate for EffectPoolTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Effects".into()
    }

    fn is_occupied(&self, slot_id: u32, cx: &App) -> bool {
        self.effect(slot_id, cx).is_some()
    }

    fn occupied_content(&self, slot_id: u32, cx: &App) -> impl IntoElement {
        let label =
            self.effect(slot_id, cx).map(|effect| effect.name()).unwrap_or("<unknown>").to_string();
        h_flex().justify_center().size_full().child(label)
    }

    fn on_activate_slot(&mut self, _slot_id: u32, _window: &mut Window, _cx: &mut App) {}
}
