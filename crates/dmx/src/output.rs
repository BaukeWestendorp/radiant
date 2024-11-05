use crate::{DmxChannel, DmxUniverse};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct DmxOutput {
    universes: HashMap<u16, DmxUniverse>,
}

impl DmxOutput {
    pub fn new() -> Self {
        Self {
            universes: HashMap::new(),
        }
    }

    pub fn get_universe(&self, universe: u16) -> Option<&DmxUniverse> {
        self.universes.get(&universe)
    }

    pub fn get_channel_value(&self, universe: u16, channel: DmxChannel) -> Option<u8> {
        Some(self.get_universe(universe)?.get_channel_value(channel))
    }

    pub fn set_channel_value(&mut self, universe: u16, channel: DmxChannel, value: u8) {
        if self.get_universe(universe).is_none() {
            self.create_universe(universe);
        }

        self.universes
            .get_mut(&universe)
            .unwrap()
            .set_channel_value(channel, value);
    }

    pub fn create_universe(&mut self, universe: u16) {
        self.universes.insert(universe, DmxUniverse::new());
    }
}
