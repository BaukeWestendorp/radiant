use gpui::{App, IntoElement, SharedString, Window, prelude::*};
use rd_core::object::{Group, Object, ObjectKind, ObjectReference, SlotId};
use rd_ui::{PoolTileDelegate, h_flex};

use crate::state::AppState;

pub struct GroupPoolTile {}

impl GroupPoolTile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn group<'a>(&self, slot_id: u32, cx: &'a App) -> Option<&'a Group> {
        AppState::engine(cx)
            .objects()
            .get(ObjectReference::Slot(ObjectKind::Group, SlotId::new(slot_id).unwrap()))
    }
}

impl PoolTileDelegate for GroupPoolTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Groups".into()
    }

    fn is_occupied(&self, slot_id: u32, cx: &App) -> bool {
        self.group(slot_id, cx).is_some()
    }

    fn occupied_content(&self, slot_id: u32, cx: &App) -> impl IntoElement {
        let label =
            self.group(slot_id, cx).map(|group| group.name()).unwrap_or("<unknown>").to_string();
        h_flex().justify_center().size_full().child(label)
    }

    fn on_activate_slot(&mut self, _slot_id: u32, _window: &mut Window, _cx: &mut App) {}
}
