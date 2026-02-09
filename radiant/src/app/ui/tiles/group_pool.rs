use std::collections::HashMap;

use gpui::{App, Entity, ReadGlobal as _, SharedString, UpdateGlobal as _, Window};
use rui::PoolTileDelegate;

use crate::{
    app::state::AppState,
    object::{Group, GroupId},
};

pub struct GroupsPoolTile {}

impl GroupsPoolTile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn groups<'a>(&self, cx: &'a App) -> &'a Entity<HashMap<GroupId, Group>> {
        AppState::global(cx).show().groups()
    }

    pub fn group<'a>(&self, slot_id: u32, cx: &'a App) -> Option<&'a Group> {
        self.groups(cx).read(cx).get(&slot_id)
    }
}

impl PoolTileDelegate for GroupsPoolTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Groups".into()
    }

    fn is_occupied(&self, slot_id: u32, cx: &App) -> bool {
        self.group(slot_id, cx).is_some()
    }

    fn occupied_label(&self, slot_id: u32, cx: &App) -> String {
        self.group(slot_id, cx).map(|g| g.name()).unwrap_or("<unknown>").to_string()
    }

    fn on_activate_slot(&mut self, slot_id: u32, _window: &mut Window, cx: &mut App) {
        let Some(fixture_ids) = self.group(slot_id, cx).map(|g| g.fixture_ids().to_vec()) else {
            return;
        };

        AppState::update_global(cx, |state, cx| {
            state.show().set_selection(fixture_ids, cx);
        })
    }
}
