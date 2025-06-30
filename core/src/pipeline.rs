use std::collections::HashMap;

use crate::{Attribute, AttributeValue, FixtureId, Patch};

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
    /// Unresolved attribute values that have been set.
    /// These will be piped down into the unresolved [Multiverse].
    attribute_values: HashMap<(FixtureId, Attribute), AttributeValue>,
    /// Unresolved direct DMX values that have been set.
    dmx_values: HashMap<dmx::Address, dmx::Value>,
    /// Once [Pipeline::resolve] has been called,
    /// all unresolved representations will be flushed into this [Multiverse].
    resolved_multiverse: dmx::Multiverse,
    resolved_attribute_values: HashMap<(FixtureId, Attribute), AttributeValue>,
    resolved_dmx_values: HashMap<dmx::Address, dmx::Value>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self::default()
    }

    /// Clears all unresolved representations.
    pub fn clear_unresolved(&mut self) {
        self.dmx_values.clear();
        self.attribute_values.clear();
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

    /// Gets an unresolved [AttributeValue] for a specific [Attribute]
    /// on a fixture with the given [FixtureId].
    pub fn get_attribute_value(
        &self,
        fixture_id: FixtureId,
        attribute: &Attribute,
    ) -> Option<AttributeValue> {
        self.attribute_values.get(&(fixture_id, attribute.clone())).copied()
    }

    /// Inserts a specific [dmx::Value] at the given [dmx::Address]
    /// to be resolved in the future.
    pub fn set_dmx_value(&mut self, address: dmx::Address, value: dmx::Value) {
        self.dmx_values.insert(address, value);
    }

    fn resolve_default_values(&mut self, patch: &Patch) {
        for fixture in patch.fixtures() {
            for (channel, value) in fixture.get_default_channel_values() {
                let address = dmx::Address::new(fixture.address().universe, channel);
                self.resolved_multiverse.set_value(&address, value);
            }
        }
    }

    fn resolve_attribute_values(&mut self, patch: &Patch) {
        for ((fixture_id, attribute), value) in self.attribute_values.clone() {
            let Some(fixture) = patch.fixture(fixture_id) else { continue };
            let Ok(values) = fixture.get_channel_values(&attribute, &value) else {
                continue;
            };

            for (channel, value) in values {
                let address = dmx::Address::new(fixture.address().universe, channel);
                self.resolved_multiverse.set_value(&address, value);
            }
        }
        self.resolved_attribute_values = self.attribute_values.clone();
    }

    fn resolve_direct_dmx_values(&mut self) {
        for (address, value) in &self.dmx_values {
            self.resolved_multiverse.set_value(address, *value);
        }
        self.resolved_dmx_values = self.dmx_values.clone();
    }

    /// Resolves all unresolved representations into the resolved [Multiverse].
    ///
    /// You can get the resolved [Multiverse] with [Pipeline::output_multiverse].
    pub fn resolve(&mut self, patch: &Patch) {
        self.resolve_default_values(patch);
        self.resolve_attribute_values(patch);
        self.resolve_direct_dmx_values();
    }

    /// Gets the resolved [Multiverse]. This will not be cleared by [Pipeline::clear_unresolved].
    pub fn resolved_multiverse(&self) -> &dmx::Multiverse {
        &self.resolved_multiverse
    }

    /// Gets the resolved [AttributeValue]s. This will not be cleared by [Pipeline::clear_unresolved].
    ///
    /// This function does not return defaults, just changed values.
    pub fn resolved_attribute_values(&self) -> &HashMap<(FixtureId, Attribute), AttributeValue> {
        &self.resolved_attribute_values
    }

    /// Merges all relevant, unresolved data from this [Pipeline] into another.
    pub fn merge_into(&self, other: &mut Pipeline) {
        for ((fixture_id, attribute), value) in &self.attribute_values {
            other.attribute_values.insert((*fixture_id, attribute.clone()), *value);
        }

        for (address, value) in &self.dmx_values {
            other.dmx_values.insert(*address, *value);
        }
    }
}
