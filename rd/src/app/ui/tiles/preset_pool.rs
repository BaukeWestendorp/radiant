use std::num::NonZeroU32;

use gpui::{App, Entity, IntoElement, SharedString, Window, prelude::*};
use rd_ui::{PoolTileDelegate, h_flex};

use rd_engine::{
    cmd::Command,
    event::Event,
    object::{Object as _, ObjectCollection, ObjectKind, Preset, PresetKind, Slot},
};

use crate::engine::EngineAppExt;

pub struct PresetPoolTile {
    kind: PresetKind,

    presets: Entity<ObjectCollection<Preset>>,
}

impl PresetPoolTile {
    pub fn new(kind: PresetKind, cx: &mut Context<Self>) -> Self {
        let presets = cx.new(|cx| cx.engine_snapshot().objects().presets(kind).clone());
        cx.on_engine_event({
            let presets = presets.clone();
            move |event, cx| match event {
                Event::ObjectChanged { kind: ObjectKind::Preset(event_preset_kind), .. }
                    if *event_preset_kind == kind =>
                {
                    let new_presets = cx.engine_snapshot().objects().presets(kind).clone();
                    presets.write(cx, new_presets);
                }
                _ => {}
            }
        })
        .detach();

        Self { kind, presets }
    }

    pub fn preset<'a>(&self, slot: u32, cx: &'a App) -> anyhow::Result<&'a Preset> {
        let slot = Slot::new(NonZeroU32::new(slot).unwrap());
        self.presets.read(cx).get_by_slot(&slot)
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
        let preset = match self.preset(slot, cx) {
            Ok(preset) => preset,
            Err(err) => {
                log::warn!("{err}");
                return;
            }
        };

        cx.execute_engine_cmd(Command::Activate {
            object_kind: ObjectKind::Preset(self.kind),
            object_id: preset.id(),
        })
    }
}
