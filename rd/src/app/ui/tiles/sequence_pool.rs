use std::num::NonZeroU32;

use gpui::{App, Entity, IntoElement, SharedString, Window, prelude::*};
use rd_engine::{
    cmd::Command,
    event::Event,
    object::{Object as _, ObjectCollection, ObjectKind, Sequence, Slot},
};
use rd_ui::{PoolTileDelegate, h_flex};

use crate::engine::EngineAppExt;

pub struct SequencePoolTile {
    sequences: Entity<ObjectCollection<Sequence>>,
}

impl SequencePoolTile {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let sequences = cx.new(|cx| cx.engine_snapshot().objects().sequences().clone());
        cx.on_engine_event({
            let sequences = sequences.clone();
            move |event, cx| match event {
                Event::ObjectChanged { kind: ObjectKind::Sequence, .. } => {
                    let new_sequences = cx.engine_snapshot().objects().sequences().clone();
                    sequences.write(cx, new_sequences);
                }
                _ => {}
            }
        })
        .detach();

        Self { sequences }
    }

    pub fn sequence<'a>(&self, slot: u32, cx: &'a App) -> anyhow::Result<&'a Sequence> {
        let slot = Slot::new(NonZeroU32::new(slot).unwrap());
        self.sequences.read(cx).get_by_slot(&slot)
    }
}

impl PoolTileDelegate for SequencePoolTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Sequences".into()
    }

    fn is_occupied(&self, slot: u32, cx: &App) -> bool {
        self.sequence(slot, cx).is_ok()
    }

    fn occupied_content(&self, slot: u32, cx: &App) -> impl IntoElement {
        let label = self
            .sequence(slot, cx)
            .map(|sequence| sequence.name())
            .unwrap_or("<unknown>")
            .to_string();
        h_flex().justify_center().size_full().child(label)
    }

    fn on_activate_slot(&mut self, slot: u32, _window: &mut Window, cx: &mut App) {
        let sequence = match self.sequence(slot, cx) {
            Ok(sequence) => sequence,
            Err(err) => {
                log::warn!("{err}");
                return;
            }
        };

        cx.execute_engine_cmd(Command::Activate {
            object_kind: ObjectKind::Sequence,
            object_id: sequence.id(),
        })
    }
}
