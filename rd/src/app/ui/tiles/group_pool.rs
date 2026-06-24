use std::num::NonZeroU32;

use gpui::{App, Entity, IntoElement, SharedString, Window, prelude::*};
use rd_engine::{
    cmd::Command,
    event::Event,
    object::{Group, Object as _, ObjectCollection, ObjectKind, Slot},
};
use rd_ui::{PoolTileDelegate, h_flex};

use crate::engine::EngineAppExt;

pub struct GroupPoolTile {
    groups: Entity<ObjectCollection<Group>>,
}

impl GroupPoolTile {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let groups = cx.new(|cx| cx.engine_snapshot().objects().groups().clone());
        cx.on_engine_event({
            let groups = groups.clone();
            move |event, cx| match event {
                Event::ObjectChanged { object_kind: ObjectKind::Group, .. } => {
                    let new_groups = cx.engine_snapshot().objects().groups().clone();
                    groups.write(cx, new_groups);
                }
                _ => {}
            }
        })
        .detach();

        Self { groups }
    }

    pub fn group<'a>(&self, slot: u32, cx: &'a App) -> anyhow::Result<&'a Group> {
        let slot = Slot::new(NonZeroU32::new(slot).unwrap());
        self.groups.read(cx).get_by_slot(&slot)
    }
}

impl PoolTileDelegate for GroupPoolTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Group".into()
    }

    fn is_occupied(&self, slot: u32, cx: &App) -> bool {
        self.group(slot, cx).is_ok()
    }

    fn occupied_content(&self, slot: u32, cx: &App) -> impl IntoElement {
        let label =
            self.group(slot, cx).map(|group| group.name()).unwrap_or("<unknown>").to_string();
        h_flex().justify_center().size_full().child(label)
    }

    fn on_activate_slot(&mut self, slot: u32, _window: &mut Window, cx: &mut App) {
        let group = match self.group(slot, cx) {
            Ok(group) => group,
            Err(err) => {
                log::warn!("{err}");
                return;
            }
        };

        cx.execute_engine_cmd(Command::Activate {
            object_kind: ObjectKind::Group,
            object_id: group.id(),
        })
    }
}
