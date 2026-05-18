use gpui::{App, IntoElement, ReadGlobal, SharedString, Window, prelude::*};
use rd_ui::{PoolTileDelegate, h_flex};

use crate::engine::{Engine, Group, Object as _, ObjectKind, ObjectReference, SlotId};

pub struct GroupPoolTile {}

impl GroupPoolTile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn group<'a>(&self, slot_id: u32, cx: &'a App) -> Option<&'a Group> {
        Engine::global(cx)
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

    fn on_activate_slot(&mut self, slot_id: u32, _window: &mut Window, cx: &mut App) {
        let Some(group) = self.group(slot_id, cx) else {
            log::error!("Tried to select group in slot {slot_id}, but it was not found");
            return;
        };

        let fixtures = group.fixture_ids().to_vec();
        Engine::global(cx).selected_fixtures().clone().update(cx, |selection, cx| {
            // FIXME: Make this a command.
            *selection = fixtures;
            cx.notify();
        });
    }
}
