use std::collections::HashMap;

use crate::attr::{Attribute, AttributeValue};
use crate::builtin::{FixtureId, Patch};

#[derive(Debug, Default, Clone)]
pub struct Pipeline {
    values: HashMap<(FixtureId, Attribute), AttributeValue>,
    resolved_values: HashMap<(FixtureId, Attribute), AttributeValue>,
    resolved_multiverse: dmx::Multiverse,
}

impl Pipeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_value(
        &mut self,
        fixture_id: FixtureId,
        attribute: Attribute,
        value: AttributeValue,
    ) {
        self.values.insert((fixture_id, attribute), value);
    }

    pub fn resolve(&mut self, patch: &Patch) {
        for ((fixture_id, attribute), value) in self.values.clone() {
            let Some(fixture) = patch.fixture(fixture_id) else { continue };
            let Ok(values) = fixture.get_channel_values(&attribute, &value, patch) else {
                continue;
            };

            for (channel, value) in values {
                let address = dmx::Address::new(fixture.address.universe, channel);
                self.resolved_multiverse.set_value(&address, value);
            }
        }
        self.resolved_values = self.values.clone();
    }

    pub fn resolved_multiverse(&self) -> &dmx::Multiverse {
        &self.resolved_multiverse
    }

    pub fn merge(&mut self, other: &Pipeline) {
        for (key, value) in &other.values {
            self.values.insert(key.clone(), value.clone());
        }
    }
}
