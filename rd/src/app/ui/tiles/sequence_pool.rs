use std::num::NonZeroU32;

use gpui::{App, IntoElement, SharedString, Window, prelude::*};
use rd_engine::{
    cmd::Command,
    object::{Object as _, ObjectKind, Sequence, Slot},
};
use rd_ui::{PoolTileDelegate, h_flex};

use crate::engine::EngineManager;

pub struct SequencesPoolTile {}

impl SequencesPoolTile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn sequence<'a>(&self, slot: u32, cx: &'a App) -> anyhow::Result<&'a Sequence> {
        EngineManager::read_snapshot(cx)
            .objects()
            .sequences()
            .get_by_slot(&Slot::new(NonZeroU32::new(slot).unwrap()))
    }
}

impl PoolTileDelegate for SequencesPoolTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Cue Lists".into()
    }

    fn is_occupied(&self, slot: u32, cx: &App) -> bool {
        self.sequence(slot, cx).is_ok()
    }

    fn occupied_content(&self, slot: u32, cx: &App) -> impl IntoElement {
        let label = self
            .sequence(slot, cx)
            .map(|sequence| sequence.name())
            .unwrap_or("<unknown>")
            .to_string();
        h_flex().justify_center().size_full().child(label)
    }

    fn on_activate_slot(&mut self, slot: u32, _window: &mut Window, cx: &mut App) {
        let slot = Slot::new(NonZeroU32::new(slot).unwrap());
        let Ok(sequence) = EngineManager::read_snapshot(cx).objects().sequences().get_by_slot(&slot)
        else {
            return;
        };
        EngineManager::execute(
            cx,
            Command::Activate { object_kind: ObjectKind::Sequence, object_id: sequence.id() },
        )
    }
}
