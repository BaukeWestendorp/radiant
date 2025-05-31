//! In Radiant, the processing pipeline creates a few layers of abstraction
//! over the DMX output:
//!
//! - Layer 4: Presets
//! - Layer 3: Attribute values
//! - Layer 2: Floating DMX
//! - Layer 1: DMX512
//!
//! After setting all values you want to output using the different layers, you can
//! flush the pipeline into the output [Multiverse][dmx::Multiverse].
//! Then, the [ProtocolManager][crate::protocols::ProtocolManager] will pick up
//! the output multiverse and send it to every output protocol.

use crate::show::{AnyPresetAssetId, Attr, Attribute, FixtureId, FloatingDmxValue, Preset, Show};
use gpui::{App, Entity, ReadGlobal};
use std::collections::HashMap;

/// The main processing pipeline.
pub struct Pipeline {
    /// The multiverse the pipeline should use to flush DMX values to.
    output_multiverse: Entity<dmx::Multiverse>,
    /// Pending DMX values to flush.
    pending_values: HashMap<dmx::Address, dmx::Value>,
    /// Pending attribute values to flush.
    pending_attr_values: HashMap<FixtureId, HashMap<Attr, FloatingDmxValue>>,
}

impl Pipeline {
    /// Creates a new [Pipeline].
    pub fn new(output_multiverse: Entity<dmx::Multiverse>) -> Self {
        Self {
            output_multiverse,
            pending_values: HashMap::new(),
            pending_attr_values: HashMap::new(),
        }
    }

    /// The [Multiverse][dmx::Multiverse] the pipeline will flush DMX values to.
    pub fn output_multiverse(&self) -> &Entity<dmx::Multiverse> {
        &self.output_multiverse
    }

    /// Directly apply a DMX value. (Layer 1).
    pub fn apply_value(&mut self, address: dmx::Address, value: dmx::Value) {
        self.pending_values.insert(address, value);
    }

    /// Apply an attribute for a specific fixture. (Layer 3).
    pub fn apply_attribute(
        &mut self,
        attribute: Attr,
        value: FloatingDmxValue,
        fixture_id: FixtureId,
    ) {
        self.pending_attr_values
            .entry(fixture_id)
            .or_default()
            .insert(attribute, value);
    }

    /// Apply a preset for a specific fixture. (Layer 4).
    pub fn apply_preset(&mut self, preset_id: AnyPresetAssetId, fixture_id: FixtureId, cx: &App) {
        let show = Show::global(cx);
        match preset_id {
            AnyPresetAssetId::Dimmer(id) => match show.assets.dimmer_presets.get(&id) {
                Some(preset) => match &preset.read(cx).data {
                    Preset::Universal(values) => {
                        for (attr, value) in values.iter() {
                            self.apply_attribute(attr.to_attr(), *value, fixture_id);
                        }
                    }
                },
                None => {}
            },
            AnyPresetAssetId::Position(id) => match show.assets.position_presets.get(&id) {
                Some(preset) => match &preset.read(cx).data {
                    Preset::Universal(values) => {
                        for (attr, value) in values.iter() {
                            self.apply_attribute(attr.to_attr(), *value, fixture_id);
                        }
                    }
                },
                None => {}
            },
            AnyPresetAssetId::Gobo(id) => match show.assets.gobo_presets.get(&id) {
                Some(preset) => match &preset.read(cx).data {
                    Preset::Universal(values) => {
                        for (attr, value) in values.iter() {
                            self.apply_attribute(attr.to_attr(), *value, fixture_id);
                        }
                    }
                },
                None => {}
            },
            AnyPresetAssetId::Color(id) => match show.assets.color_presets.get(&id) {
                Some(preset) => match &preset.read(cx).data {
                    Preset::Universal(values) => {
                        for (attr, value) in values.iter() {
                            self.apply_attribute(attr.to_attr(), *value, fixture_id);
                        }
                    }
                },
                None => {}
            },
            AnyPresetAssetId::Beam(id) => match show.assets.beam_presets.get(&id) {
                Some(preset) => match &preset.read(cx).data {
                    Preset::Universal(values) => {
                        for (attr, value) in values.iter() {
                            self.apply_attribute(attr.to_attr(), *value, fixture_id);
                        }
                    }
                },
                None => {}
            },
            AnyPresetAssetId::Focus(id) => match show.assets.focus_presets.get(&id) {
                Some(preset) => match &preset.read(cx).data {
                    Preset::Universal(values) => {
                        for (attr, value) in values.iter() {
                            self.apply_attribute(attr.to_attr(), *value, fixture_id);
                        }
                    }
                },
                None => {}
            },
            AnyPresetAssetId::Control(id) => match show.assets.control_presets.get(&id) {
                Some(preset) => match &preset.read(cx).data {
                    Preset::Universal(values) => {
                        for (attr, value) in values.iter() {
                            self.apply_attribute(attr.to_attr(), *value, fixture_id);
                        }
                    }
                },
                None => {}
            },
            AnyPresetAssetId::Shapers(id) => match show.assets.shapers_presets.get(&id) {
                Some(preset) => match &preset.read(cx).data {
                    Preset::Universal(values) => {
                        for (attr, value) in values.iter() {
                            self.apply_attribute(attr.to_attr(), *value, fixture_id);
                        }
                    }
                },
                None => {}
            },
            AnyPresetAssetId::Video(id) => match show.assets.video_presets.get(&id) {
                Some(preset) => match &preset.read(cx).data {
                    Preset::Universal(values) => {
                        for (attr, value) in values.iter() {
                            self.apply_attribute(attr.to_attr(), *value, fixture_id);
                        }
                    }
                },
                None => {}
            },
        };
    }

    fn flush_default_values(&mut self, cx: &mut App) -> anyhow::Result<()> {
        let patch = Show::global(cx).patch.read(cx);

        let mut values = Vec::<(dmx::Address, dmx::Value)>::new();

        for fixture in patch.fixtures() {
            for channel in &fixture.dmx_mode(patch).dmx_channels {
                let Some((_, channel_function)) = channel.initial_function() else {
                    continue;
                };

                let Some(offsets) = &channel.offset else { continue };

                let default_bytes = match &channel_function.default.bytes().get() {
                    1 => channel_function.default.to_u8().to_be_bytes().to_vec(),
                    2 => channel_function.default.to_u16().to_be_bytes().to_vec(),
                    4 => channel_function.default.to_u32().to_be_bytes().to_vec(),
                    _ => panic!("Unsupported default value size"),
                };

                for (i, offset) in offsets.iter().enumerate() {
                    let address = fixture.address().with_channel_offset(*offset as u16 - 1)?;
                    let value = dmx::Value(default_bytes[i]);
                    values.push((address, value));
                }
            }
        }

        self.flush_values(values, cx);

        Ok(())
    }

    /// Flush all pending values to the output [Multiverse][dmx::Multiverse].
    pub fn flush(&mut self, cx: &mut App) -> anyhow::Result<()> {
        self.flush_default_values(cx)?;
        self.flush_pending_attributes(cx);
        self.flush_pending_values(cx);
        Ok(())
    }

    fn flush_pending_attributes(&mut self, cx: &mut App) {
        for (fixture_id, values) in self.pending_attr_values.clone().iter() {
            for (attr, value) in values.iter() {
                self.flush_attribute(*attr, *value, *fixture_id, cx);
            }
        }

        self.pending_attr_values.clear();
    }

    fn flush_attribute(
        &mut self,
        attr: Attr,
        value: FloatingDmxValue,
        fixture_id: FixtureId,
        cx: &mut App,
    ) {
        let patch = Show::global(cx).patch.read(cx);

        let Some(fixture) = patch.fixture(fixture_id).cloned() else {
            return;
        };

        let Some(offset) = fixture.channel_offset_for_attr(&attr.to_string(), patch).cloned()
        else {
            return;
        };

        let value_bytes = match offset.len() {
            1 => {
                let byte_value = (value.0 * 0xff as f32) as u8;
                vec![byte_value]
            }
            2 => {
                let int_value = (value.0 * 0xffff as f32) as u16;
                vec![(int_value >> 8) as u8, (int_value & 0xFF) as u8]
            }
            3 => {
                let int_value = (value.0 * 0xffffff as f32) as u32;
                vec![
                    (int_value >> 16) as u8,
                    ((int_value >> 8) & 0xFF) as u8,
                    (int_value & 0xFF) as u8,
                ]
            }
            4 => {
                let int_value = (value.0 * 0xffffffff_u32 as f32) as u32;
                vec![
                    (int_value >> 24) as u8,
                    ((int_value >> 16) & 0xFF) as u8,
                    ((int_value >> 8) & 0xFF) as u8,
                    (int_value & 0xFF) as u8,
                ]
            }
            _ => vec![0],
        };

        for (byte, offset) in value_bytes.iter().zip(&offset) {
            let address = fixture.address().with_channel_offset(*offset as u16 - 1).unwrap();
            self.output_multiverse.update(cx, |multiverse, cx| {
                multiverse.set_value(&address, dmx::Value(*byte));
                cx.notify();
            });
        }
    }

    fn flush_pending_values(&mut self, cx: &mut App) {
        for (addr, value) in self.pending_values.clone().iter() {
            self.flush_value(addr, *value, cx);
        }

        self.pending_values.clear();
    }

    fn flush_values(&self, values: Vec<(dmx::Address, dmx::Value)>, cx: &mut App) {
        self.output_multiverse.update(cx, |multiverse, cx| {
            for (addr, value) in values {
                multiverse.set_value(&addr, value);
            }
            cx.notify();
        });
    }

    fn flush_value(&self, addr: &dmx::Address, value: dmx::Value, cx: &mut App) {
        self.output_multiverse.update(cx, |multiverse, cx| {
            multiverse.set_value(addr, value);

            cx.notify();
        });
    }
}
