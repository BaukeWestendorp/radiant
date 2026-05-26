use std::num::NonZeroU32;

use gpui::{App, IntoElement, SharedString, Window, prelude::*};
use rd_ui::{PoolTileDelegate, h_flex};

use rd_engine::{Command, Group, Object as _, Slot};

use crate::engine::EngineManager;

pub struct GroupPoolTile {}

impl GroupPoolTile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn group<'a>(&self, slot: u32, cx: &'a App) -> anyhow::Result<&'a Group> {
        EngineManager::snapshot(cx)
            .objects()
            .groups()
            .get_by_slot(&Slot::new(NonZeroU32::new(slot).unwrap()))
    }
}

impl PoolTileDelegate for GroupPoolTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Groups".into()
    }

    fn is_occupied(&self, slot: u32, cx: &App) -> bool {
        self.group(slot, cx).is_ok()
    }

    fn occupied_content(&self, slot: u32, cx: &App) -> impl IntoElement {
        let name = match self.group(slot, cx) {
            Ok(group) => group.name().to_string(),
            Err(_) => "<unknown>".to_string(),
        };

        h_flex().justify_center().size_full().child(name)
    }

    fn on_activate_slot(&mut self, slot: u32, window: &mut Window, cx: &mut App) {
        let slot = Slot::new(NonZeroU32::new(slot).unwrap());

        let Ok(group) = EngineManager::snapshot(cx).objects().groups().get_by_slot(&slot) else {
            log::error!("Tried to select group in slot {slot}, but it was not found");
            return;
        };

        let fixture_ids = group.fixture_ids().to_vec();
        match window.modifiers().shift {
            true => EngineManager::execute(cx, Command::SelectionAdd { fixture_ids }),
            false => EngineManager::execute(cx, Command::SelectionSet { fixture_ids }),
        }
    }
}
