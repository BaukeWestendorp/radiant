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

    pub fn set_value(&mut self, fid: FixtureId, attribute: Attribute, value: AttributeValue) {
        self.values.insert((fid, attribute), value);
    }

    pub fn resolve(&mut self, patch: &Patch) {
        for ((fid, attribute), value) in self.values.clone() {
            let Some(fixture) = patch.fixture(fid) else { continue };
            let Ok(values) = fixture.get_channel_values(&attribute, &value, patch) else {
                continue;
            };

            let Some(address) = fixture.address else {
                log::error!("fixture does not have an address");
                continue;
            };

            for (channel, value) in values {
                let address = dmx::Address::new(address.universe, channel);
                self.resolved_multiverse.set_value(&address, value);
            }
        }
        self.resolved_values = self.values.clone();
    }

    pub fn resolved_multiverse(&self) -> &dmx::Multiverse {
        &self.resolved_multiverse
    }
}
