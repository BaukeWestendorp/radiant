use std::num::NonZeroU32;

use gpui::{App, IntoElement, SharedString, Window, prelude::*};
use rd_ui::{PoolTileDelegate, h_flex};

use rd_engine::{Command, CueList, Object, ObjectKind, Slot};

use crate::engine::EngineManager;

pub struct CueListsPoolTile {}

impl CueListsPoolTile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn cue_list<'a>(&self, slot: u32, cx: &'a App) -> anyhow::Result<&'a CueList> {
        EngineManager::snapshot(cx)
            .objects()
            .cue_lists()
            .get_by_slot(&Slot::new(NonZeroU32::new(slot).unwrap()))
    }
}

impl PoolTileDelegate for CueListsPoolTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Cue Lists".into()
    }

    fn is_occupied(&self, slot: u32, cx: &App) -> bool {
        self.cue_list(slot, cx).is_ok()
    }

    fn occupied_content(&self, slot: u32, cx: &App) -> impl IntoElement {
        let label = self
            .cue_list(slot, cx)
            .map(|cue_list| cue_list.name())
            .unwrap_or("<unknown>")
            .to_string();
        h_flex().justify_center().size_full().child(label)
    }

    fn on_activate_slot(&mut self, slot: u32, _window: &mut Window, cx: &mut App) {
        let slot = Slot::new(NonZeroU32::new(slot).unwrap());
        let Ok(cue_list) = EngineManager::snapshot(cx).objects().cue_lists().get_by_slot(&slot)
        else {
            return;
        };
        EngineManager::execute(
            cx,
            Command::Activate { object_kind: ObjectKind::CueList, object_id: cue_list.id() },
        )
    }
}
