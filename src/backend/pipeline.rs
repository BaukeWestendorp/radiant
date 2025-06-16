//! # Pipeline
//!
//! ``` markdown
//! - Resolve Presets            (e.g. Executors)
//! - Resolve Attribute Values   (e.g. Programmer, Presets >>)
//! - Resolve Direct DMX Values  (e.g. Attribute Values >>)
//! - Output DMX                 (e.g. Via sACN)
//! ```

use std::collections::HashMap;

use crate::{
    backend::{
        patch::{
            Patch,
            attr::{Attribute, AttributeValue},
            fixture::FixtureId,
        },
        preset::Preset,
    },
    dmx::{self, Multiverse},
};

#[derive(Default)]
pub struct Pipeline {
    presets: Vec<Preset>,
    attribute_values: HashMap<(FixtureId, Attribute), AttributeValue>,
    unresolved_multiverse: Multiverse,
    resolved_multiverse: Multiverse,
}

impl Pipeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.unresolved_multiverse.clear();
        self.attribute_values.clear();
        self.presets.clear();
    }

    pub fn set_preset(&mut self, preset: Preset) {
        self.presets.push(preset);
    }

    pub fn set_attribute_value(
        &mut self,
        fixture_id: FixtureId,
        attribute: Attribute,
        value: AttributeValue,
    ) {
        self.attribute_values.insert((fixture_id, attribute), value);
    }

    pub fn set_dmx_value(&mut self, address: &dmx::Address, value: dmx::Value) {
        self.unresolved_multiverse.set_value(address, value);
    }

    fn resolve_default_values(&mut self, patch: &Patch) {
        for fixture in patch.fixtures() {
            for (channel, value) in fixture.get_default_channel_values() {
                let address = dmx::Address::new(fixture.address.universe, channel);
                self.set_dmx_value(&address, value);
            }
        }
    }

    fn resolve_presets(&mut self) {
        for preset in self.presets.clone() {
            match preset {
                Preset::Selective(selective_preset) => {
                    for ((fixture_id, attribute), value) in selective_preset.get_attribute_values()
                    {
                        self.set_attribute_value(*fixture_id, attribute.clone(), *value);
                    }
                }
            }
        }
    }

    fn resolve_attribute_values(&mut self, patch: &Patch) {
        for ((fixture_id, attribute), value) in self.attribute_values.clone() {
            let Some(fixture) = patch.fixture(&fixture_id) else { continue };
            let Ok(values) = fixture.get_channel_values(&attribute, &value) else {
                continue;
            };

            for (channel, value) in values {
                let address = dmx::Address::new(fixture.address.universe, channel);
                self.set_dmx_value(&address, value);
            }
        }
    }

    fn resolve_direct_dmx_values(&mut self) {
        std::mem::swap(&mut self.unresolved_multiverse, &mut self.resolved_multiverse);
    }

    pub fn resolve(&mut self, patch: &Patch) {
        self.resolve_default_values(patch);
        self.resolve_presets();
        self.resolve_attribute_values(patch);
        self.resolve_direct_dmx_values();
    }

    pub fn output_multiverse(&self) -> &Multiverse {
        &self.resolved_multiverse
    }
}
