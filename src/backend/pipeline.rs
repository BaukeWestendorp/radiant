use std::collections::HashMap;

use crate::backend::object::PresetContent;
use crate::backend::patch::Patch;
use crate::backend::patch::attr::Attribute;
use crate::backend::patch::attr::AttributeValue;
use crate::backend::patch::fixture::FixtureId;
use crate::dmx::{self, Multiverse};

/// The pipeline is used to converge all different kinds
/// of representation for DMX output into a single [Multiverse].
///
/// ``` markdown
/// Layers:
/// - (4) Resolve Presets            (e.g. Executors)
/// - (3) Resolve Attribute Values   (e.g. Programmer, Presets >>)
/// - (2) Resolve Direct DMX Values  (e.g. Attribute Values >>)
/// - (1) Output DMX                 (e.g. Via sACN)
/// ```
#[derive(Debug, Default, Clone)]
pub struct Pipeline {
    /// Unresolved presets that have been set.
    /// These will be piped down into the attribute values.
    presets: Vec<PresetContent>,
    /// Unresolved attribute values that have been set.
    /// These will be piped down into the unresolved [Multiverse].
    attribute_values: HashMap<(FixtureId, Attribute), AttributeValue>,
    /// Unresolved direct DMX values that have been set.
    dmx_values: HashMap<dmx::Address, dmx::Value>,
    /// Once [Pipeline::resolve] has been called,
    /// all unresolved representations will be flushed into this [Multiverse].
    resolved_multiverse: Multiverse,
}

impl Pipeline {
    pub fn new() -> Self {
        Self::default()
    }

    /// Clears all unresolved representations.
    pub fn clear(&mut self) {
        self.dmx_values.clear();
        self.attribute_values.clear();
        self.presets.clear();
    }

    /// Inserts a [Preset] to be resolved in the future.
    pub fn set_preset(&mut self, preset: PresetContent) {
        self.presets.push(preset);
    }

    /// Inserts an [AttributeValue] for a specific [Attribute]
    /// on a fixture with the given [FixtureId]
    /// to be resolved in the future.
    pub fn set_attribute_value(
        &mut self,
        fixture_id: FixtureId,
        attribute: Attribute,
        value: AttributeValue,
    ) {
        self.attribute_values.insert((fixture_id, attribute), value);
    }

    /// Inserts a specific [dmx::Value] at the given [dmx::Address]
    /// to be resolved in the future.
    pub fn set_dmx_value(&mut self, address: dmx::Address, value: dmx::Value) {
        self.dmx_values.insert(address, value);
    }

    fn resolve_default_values(&mut self, patch: &Patch) {
        for fixture in patch.fixtures() {
            for (channel, value) in fixture.get_default_channel_values() {
                let address = dmx::Address::new(fixture.address.universe, channel);
                self.resolved_multiverse.set_value(&address, value);
            }
        }
    }

    fn resolve_presets(&mut self) {
        for preset in self.presets.clone() {
            match preset {
                PresetContent::Selective(selective_preset) => {
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
                self.resolved_multiverse.set_value(&address, value);
            }
        }
    }

    fn resolve_direct_dmx_values(&mut self) {
        for (address, value) in &self.dmx_values {
            self.resolved_multiverse.set_value(address, *value);
        }
    }

    /// Resolves all unresolved representations into the resolved [Multiverse].
    ///
    /// You can get the resolved [Multiverse] with [Pipeline::output_multiverse].
    pub fn resolve(&mut self, patch: &Patch) {
        self.resolve_default_values(patch);
        self.resolve_presets();
        self.resolve_attribute_values(patch);
        self.resolve_direct_dmx_values();
    }

    /// Gets the resolved [Multiverse]. This will not be cleared by [Pipeline::clear].
    pub fn output_multiverse(&self) -> &Multiverse {
        &self.resolved_multiverse
    }

    /// Merges all relevant, unresolved data from this [Pipeline] into another.
    pub fn merge_into(&self, other: &mut Pipeline) {
        for preset in &self.presets {
            other.presets.push(preset.clone());
        }

        for ((fixture_id, attribute), value) in &self.attribute_values {
            other.attribute_values.insert((*fixture_id, attribute.clone()), value.clone());
        }

        for (address, value) in &self.dmx_values {
            other.dmx_values.insert(*address, *value);
        }
    }
}
