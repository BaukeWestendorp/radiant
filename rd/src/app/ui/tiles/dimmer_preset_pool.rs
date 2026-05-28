use std::num::NonZeroU32;

use gpui::{App, IntoElement, SharedString, Window, prelude::*};
use rd_ui::{PoolTileDelegate, h_flex};

use rd_engine::{DimmerPreset, Object as _, Slot};

use crate::engine::EngineManager;

pub struct DimmerPresetPoolTile {}

impl DimmerPresetPoolTile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn dimmer_preset<'a>(&self, slot: u32, cx: &'a App) -> anyhow::Result<&'a DimmerPreset> {
        EngineManager::snapshot(cx)
            .objects()
            .dimmer_presets()
            .get_by_slot(&Slot::new(NonZeroU32::new(slot).unwrap()))
    }
}

impl PoolTileDelegate for DimmerPresetPoolTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Dimmer Presets".into()
    }

    fn is_occupied(&self, slot: u32, cx: &App) -> bool {
        self.dimmer_preset(slot, cx).is_ok()
    }

    fn occupied_content(&self, slot: u32, cx: &App) -> impl IntoElement {
        let name = match self.dimmer_preset(slot, cx) {
            Ok(dimmer_preset) => dimmer_preset.name().to_string(),
            Err(_) => "<unknown>".to_string(),
        };

        h_flex().justify_center().size_full().child(name)
    }

    fn on_activate_slot(&mut self, slot: u32, _window: &mut Window, cx: &mut App) {
        let slot = Slot::new(NonZeroU32::new(slot).unwrap());

        let Ok(dimmer_preset) =
            EngineManager::snapshot(cx).objects().dimmer_presets().get_by_slot(&slot)
        else {
            log::error!("Tried to select dimmer_preset in slot {slot}, but it was not found");
            return;
        };

        log::warn!("programmer not implemented yet. tried to select '{}'", dimmer_preset.name());
    }
}
