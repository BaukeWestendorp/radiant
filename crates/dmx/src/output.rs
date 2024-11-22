//! This module contains the [DmxOutput] struct, which represents the output that can be sent to an interface.
//! It contains a collection of [DmxUniverse]s, where each universe has 512 channels, and each channel has a value between 0 and 255, represented as a [u8].

use crate::{DmxAddress, DmxChannel, DmxUniverse, DmxUniverseId};
use std::collections::HashMap;

/// A [DmxOutput] represents the output that can be sent to an interface. It contains a collection of [DmxUniverse]s.
/// Each universe has 512 channels, where each channel has a value between 0 and 255, represented as a [u8].
#[derive(Debug, Clone, Default)]
pub struct DmxOutput {
    universes: HashMap<DmxUniverseId, DmxUniverse>,
}

impl DmxOutput {
    /// Create a new, empty [DmxOutput].
    pub fn new() -> Self {
        Self {
            universes: HashMap::new(),
        }
    }

    /// Get the [DmxUniverse] with the given `universe` number if it exists.
    pub fn universe(&self, universe: DmxUniverseId) -> Option<&DmxUniverse> {
        self.universes.get(&universe)
    }

    /// Get the value at a specific `channel` in the given `universe` if that universe exists.
    pub fn channel_value(&self, universe: DmxUniverseId, channel: DmxChannel) -> Option<u8> {
        Some(self.universe(universe)?.get_channel_value(channel))
    }

    /// Set the value at a specific [DmxAddress].
    /// If the universe does not exist, it will be created.
    pub fn set_channel_value(&mut self, address: DmxAddress, value: u8) {
        if self.universe(address.universe).is_none() {
            self.create_universe(address.universe);
        }

        self.universes
            .get_mut(&address.universe)
            .expect("universe should have been created")
            .set_channel_value(address.channel, value);
    }

    /// Create a new, empty [DmxUniverse] with the given `universe` number.
    pub fn create_universe(&mut self, universe: DmxUniverseId) {
        self.universes.insert(universe, DmxUniverse::new());
    }
}
