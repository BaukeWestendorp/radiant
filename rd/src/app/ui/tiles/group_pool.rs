use std::num::NonZeroU32;

use gpui::{App, IntoElement, SharedString, Window, prelude::*};
use rd_ui::{PoolTileDelegate, h_flex};

use rd_engine::{Command, Group, Object as _, ObjectKind, Slot};

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

    fn on_activate_slot(&mut self, slot: u32, _window: &mut Window, cx: &mut App) {
        let slot = Slot::new(NonZeroU32::new(slot).unwrap());
        let Ok(group) = EngineManager::snapshot(cx).objects().groups().get_by_slot(&slot) else {
            return;
        };
        EngineManager::execute(
            cx,
            Command::Activate { object_kind: ObjectKind::Group, object_id: group.id() },
        )
    }
}
