use gpui::{App, IntoElement, SharedString, Window, prelude::*};
use rd_core::object::{CueList, Object, ObjectKind, ObjectReference, SlotId};
use rd_ui::{PoolTileDelegate, h_flex};

use crate::state::AppState;

pub struct CueListsPoolTile {}

impl CueListsPoolTile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn cue_list<'a>(&self, slot_id: u32, cx: &'a App) -> Option<&'a CueList> {
        AppState::engine(cx)
            .objects()
            .get(ObjectReference::Slot(ObjectKind::CueList, SlotId::new(slot_id).unwrap()))
    }
}

impl PoolTileDelegate for CueListsPoolTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Cue Lists".into()
    }

    fn is_occupied(&self, slot_id: u32, cx: &App) -> bool {
        self.cue_list(slot_id, cx).is_some()
    }

    fn occupied_content(&self, slot_id: u32, cx: &App) -> impl IntoElement {
        let label = self
            .cue_list(slot_id, cx)
            .map(|cue_list| cue_list.name())
            .unwrap_or("<unknown>")
            .to_string();
        h_flex().justify_center().size_full().child(label)
    }

    fn on_activate_slot(&mut self, _slot_id: u32, _window: &mut Window, _cx: &mut App) {}
}
