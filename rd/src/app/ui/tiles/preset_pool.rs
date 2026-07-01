use std::num::NonZeroU32;

use gpui::{
    App, Entity, FontWeight, IntoElement, ReadGlobal, SharedString, Window, div, prelude::*,
    relative,
};
use rd_ui::{ActiveTheme, FieldEvent, FieldState, PoolTileDelegate, Popup, PopupAppExt, h_flex};

use rd_engine::{
    cmd::{Command, StoreKind},
    event::Event,
    object::{Object as _, ObjectCollection, ObjectKind, Preset, PresetKind, Slot},
};

use crate::{
    app::state::{Mode, State},
    engine::EngineAppExt,
};

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
                Event::ObjectChanged {
                    object_kind: ObjectKind::Preset(event_preset_kind), ..
                } if *event_preset_kind == kind => {
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
        let Ok(preset) = self.preset(slot, cx) else { return div() };

        let universal = !preset.universal().is_empty();
        let global = !preset.global().is_empty();
        let selective = !preset.selective().is_empty();

        div()
            .relative()
            .size_full()
            .child(
                h_flex().size_full().absolute().justify_center().child(preset.name().to_string()),
            )
            .child(
                div()
                    .text_sm()
                    .p_1()
                    .line_height(relative(0.8))
                    .absolute()
                    .size_full()
                    .flex()
                    .flex_row_reverse()
                    .gap_1()
                    .text_color(cx.theme().fg_secondary)
                    .when(universal, |e| e.child(div().font_weight(FontWeight::BOLD).child("U")))
                    .when(global, |e| e.child(div().font_weight(FontWeight::BOLD).child("G")))
                    .when(selective, |e| e.child(div().font_weight(FontWeight::BOLD).child("S"))),
            )
    }

    fn on_activate_slot(&mut self, slot: u32, window: &mut Window, cx: &mut App) {
        let preset = match self.preset(slot, cx) {
            Ok(preset) => preset,
            Err(err) => {
                log::warn!("{err}");
                return;
            }
        };

        let mode = State::global(cx).mode();
        match mode.read(cx) {
            Mode::Normal => cx.execute_engine_cmd(Command::Activate {
                object_kind: ObjectKind::Preset(self.kind),
                object_id: preset.id(),
            }),
            Mode::Store => {
                let slot = Slot::new(NonZeroU32::new(slot).unwrap());
                cx.execute_engine_cmd(Command::Store {
                    kind: StoreKind::Preset { slot, kind: self.kind },
                });
                mode.write(cx, Mode::Normal);
            }
            Mode::Rename => {
                let object_id = preset.id();
                let object_kind = ObjectKind::Preset(self.kind);
                let input = cx.new(|cx| {
                    FieldState::<SharedString>::new("rename-field", cx.focus_handle(), window, cx)
                });
                cx.subscribe(&input, move |_, event, cx| match event {
                    FieldEvent::Submit(v) => {
                        let name = v.to_string();
                        cx.execute_engine_cmd(Command::Rename { object_id, object_kind, name });
                        mode.write(cx, Mode::Normal);
                    }
                    _ => {}
                })
                .detach();
                cx.show_popup(|cx| Popup::text("Rename Object", input, window, cx));
            }
        }
    }

    fn on_activate_empty_slot(&mut self, slot: u32, _window: &mut Window, cx: &mut App) {
        let mode = State::global(cx).mode();
        match mode.read(cx) {
            Mode::Normal => {}
            Mode::Store => {
                let slot = Slot::new(NonZeroU32::new(slot).unwrap());
                cx.execute_engine_cmd(Command::Store {
                    kind: StoreKind::Preset { slot, kind: self.kind },
                });
                mode.write(cx, Mode::Normal);
            }
            Mode::Rename => {}
        }
    }

    fn empty_slots_clickable(&self, cx: &App) -> bool {
        match State::global(cx).mode().read(cx) {
            Mode::Normal => false,
            Mode::Store => true,
            Mode::Rename => false,
        }
    }
}
