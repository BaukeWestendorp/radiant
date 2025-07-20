use std::collections::HashMap;

use crate::patch::{Attribute, AttributeValue, FixtureId, Patch};

/// The [Pipeline] resolves all representations of DMX output
/// into a single [dmx::Multiverse].
///
/// ``` markdown
/// Layers:
/// - (3) Resolve Presets            (e.g. Executors)
/// - (2) Resolve Attribute Values   (e.g. Programmer, Presets)
/// - (1) Output DMX                 (e.g. Via sACN)
/// ```
#[derive(Debug, Default, Clone)]
pub struct Pipeline {
    /// Unresolved attribute values set for specific fixtures.
    /// These are merged into the unresolved multiverse during
    /// resolution.
    attribute_values: HashMap<(FixtureId, Attribute), AttributeValue>,
    /// The resolved attribute values after [Pipeline::resolve].
    resolved_attribute_values: HashMap<(FixtureId, Attribute), AttributeValue>,
    /// The resolved DMX output after [Pipeline::resolve] is called.
    resolved_multiverse: dmx::Multiverse,
}

impl Pipeline {
    /// Creates a new, empty [Pipeline].
    pub fn new() -> Self {
        Self::default()
    }

    /// Clears all unresolved attribute values.
    ///
    /// This does not affect the resolved output.
    pub fn clear_unresolved(&mut self) {
        self.attribute_values.clear();
    }

    /// Sets an unresolved [AttributeValue] for a given [FixtureId] and
    /// [Attribute].
    ///
    /// This value will be included in the next resolution.
    pub fn set_attribute_value(
        &mut self,
        fixture_id: FixtureId,
        attribute: Attribute,
        value: AttributeValue,
    ) {
        self.attribute_values.insert((fixture_id, attribute), value);
    }

    /// Gets an unresolved [AttributeValue] for a given [FixtureId] and
    /// [Attribute], if present.
    pub fn get_attribute_value(
        &self,
        fixture_id: FixtureId,
        attribute: &Attribute,
    ) -> Option<AttributeValue> {
        self.attribute_values.get(&(fixture_id, attribute.clone())).copied()
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

    /// Resolves all unresolved values into the final [dmx::Multiverse] output.
    ///
    /// This processes default values, attribute values in order. The resolved
    /// output can be accessed with [Pipeline::resolved_multiverse].
    pub fn resolve(&mut self, patch: &Patch) {
        self.resolve_attribute_values(patch);
    }

    /// Returns the resolved [dmx::Multiverse] after the last call to
    /// [Pipeline::resolve].
    ///
    /// This output is not affected by [Pipeline::clear_unresolved].
    pub fn resolved_multiverse(&self) -> &dmx::Multiverse {
        &self.resolved_multiverse
    }

    /// Returns the resolved [AttributeValue]s after the last call to
    /// [Pipeline::resolve].
    ///
    /// Only changed values are included; defaults are not returned.
    pub fn resolved_attribute_values(&self) -> &HashMap<(FixtureId, Attribute), AttributeValue> {
        &self.resolved_attribute_values
    }

    /// Merges all unresolved data from this [Pipeline] into another [Pipeline].
    pub fn merge_unresolved_into(&self, other: &mut Pipeline) {
        for ((fixture_id, attribute), value) in &self.attribute_values {
            other.attribute_values.insert((*fixture_id, attribute.clone()), *value);
        }
    }
}
