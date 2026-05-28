use std::num::NonZeroU32;

use gpui::{App, IntoElement, SharedString, Window, prelude::*};
use rd_ui::{PoolTileDelegate, h_flex};

use rd_engine::{Object as _, Preset, PresetKind, Slot};

use crate::engine::EngineManager;

pub struct PresetPoolTile {
    kind: PresetKind,
}

impl PresetPoolTile {
    pub fn new(kind: PresetKind) -> Self {
        Self { kind }
    }

    pub fn preset<'a>(&self, slot: u32, cx: &'a App) -> anyhow::Result<&'a Preset> {
        EngineManager::snapshot(cx)
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

        let Ok(dimmer_preset) =
            EngineManager::snapshot(cx).objects().dimmer_presets().get_by_slot(&slot)
        else {
            log::error!("Tried to select preset in slot {slot}, but it was not found");
            return;
        };

        log::warn!("programmer not implemented yet. tried to select '{}'", dimmer_preset.name());
    }
}
