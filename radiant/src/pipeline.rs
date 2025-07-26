use std::collections::HashMap;

use crate::show::{Attribute, AttributeValue, FixtureId, Patch};

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
    /// Unresolved values set for specific fixtures.
    /// These are merged into the unresolved multiverse during resolution.
    values: HashMap<(FixtureId, Attribute), AttributeValue>,
    /// The resolved attribute values after [Pipeline::resolve].
    resolved_values: HashMap<(FixtureId, Attribute), AttributeValue>,
    /// The resolved DMX output after [Pipeline::resolve] is called.
    resolved_multiverse: dmx::Multiverse,
}

impl Pipeline {
    /// Creates a new, empty [Pipeline].
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets an unresolved [AttributeValue] for a given [FixtureId] and
    /// [Attribute].
    ///
    /// This value will be included in the next resolution.
    pub fn set_value(
        &mut self,
        fixture_id: FixtureId,
        attribute: Attribute,
        value: AttributeValue,
    ) {
        self.values.insert((fixture_id, attribute), value);
    }

    /// Resolves all unresolved values into the final [dmx::Multiverse] output.
    ///
    /// This processes default values, attribute values in order. The resolved
    /// output can be accessed with [Pipeline::resolved_multiverse].
    pub fn resolve(&mut self, patch: &Patch) {
        for ((fixture_id, attribute), value) in self.values.clone() {
            let Some(fixture) = patch.fixture(fixture_id) else { continue };
            let Ok(values) = fixture.get_channel_values(&attribute, &value, patch) else {
                continue;
            };

            for (channel, value) in values {
                let address = dmx::Address::new(fixture.address().universe, channel);
                self.resolved_multiverse.set_value(&address, value);
            }
        }
        self.resolved_values = self.values.clone();
    }

    /// Returns the resolved [dmx::Multiverse] after the last call to
    /// [Pipeline::resolve].
    ///
    /// This output is not affected by [Pipeline::clear_unresolved].
    pub fn resolved_multiverse(&self) -> &dmx::Multiverse {
        &self.resolved_multiverse
    }
}
