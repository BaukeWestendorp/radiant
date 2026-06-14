use std::num::NonZeroU32;

use gpui::{App, IntoElement, SharedString, Window, prelude::*};
use rd_ui::{PoolTileDelegate, h_flex};

use crate::engine::EngineManager;
use rd_engine::{
    cmd::Command,
    object::{Object as _, ObjectKind, Preset, PresetKind, Slot},
};

pub struct PresetPoolTile {
    kind: PresetKind,
}

impl PresetPoolTile {
    pub fn new(kind: PresetKind) -> Self {
        Self { kind }
    }

    pub fn preset<'a>(&self, slot: u32, cx: &'a App) -> anyhow::Result<&'a Preset> {
        EngineManager::read_snapshot(cx)
            .objects()
            .preset_by_slot(&Slot::new(NonZeroU32::new(slot).unwrap()), &self.kind)
    }
}

impl PoolTileDelegate for PresetPoolTile {
    fn title(&self, _cx: &App) -> SharedString {
        format!("{} Presets", self.kind).into()
    }

    fn is_occupied(&self, slot: u32, cx: &App) -> bool {
        self.preset(slot, cx).is_ok()
    }

    fn occupied_content(&self, slot: u32, cx: &App) -> impl IntoElement {
        let name = match self.preset(slot, cx) {
            Ok(preset) => preset.name().to_string(),
            Err(_) => "<unknown>".to_string(),
        };

        h_flex().justify_center().size_full().child(name)
    }

    fn on_activate_slot(&mut self, slot: u32, _window: &mut Window, cx: &mut App) {
        let slot = Slot::new(NonZeroU32::new(slot).unwrap());

        let Ok(preset) = EngineManager::read_snapshot(cx).objects().preset_by_slot(&slot, &self.kind)
        else {
            return;
        };

        EngineManager::execute(
            cx,
            Command::Activate {
                object_kind: ObjectKind::Preset(self.kind),
                object_id: preset.id(),
            },
        )
    }
}
