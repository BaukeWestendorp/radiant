use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DmxOutput {
    universes: Vec<DmxUniverse>,
}

impl DmxOutput {
    pub fn new() -> Self {
        Self {
            universes: Vec::new(),
        }
    }

    /// Gets all universes that are being outputted.
    pub fn universes(&self) -> &[DmxUniverse] {
        &self.universes
    }

    /// Gets a reference to a specific universe. `id` is zero-based.
    pub fn universe(&self, id: u16) -> Option<&DmxUniverse> {
        self.universes.iter().find(|u| u.id() == id)
    }

    /// Gets a mutable reference to a specific universe. `id` is zero-based.
    pub fn universe_mut(&mut self, id: u16) -> Option<&mut DmxUniverse> {
        self.universes.iter_mut().find(|u| u.id() == id)
    }

    /// Adds a universe if it does not exist. `id` id zero-based.
    ///
    /// Fails if the id of the universe is invalid.
    pub fn add_universe_if_absent(&mut self, id: u16) {
        self.universes.push(DmxUniverse::new(id))
    }

    /// Removes a universe from the output. `id` is zero-based.
    pub fn remove_universe(&mut self, id: u16) {
        self.universes.retain(|u| u.id != id);
    }

    /// Gets the value of a channel. Universe and address are zero-based.
    pub fn channel(&self, channel: DmxChannel) -> anyhow::Result<u8> {
        let Some(universe) = self.universe(channel.universe) else {
            return Err(anyhow!(
                "Failed to get universe with id {} when getting channel",
                channel.universe
            ));
        };

        universe.get_address(channel.address)
    }

    /// Sets the value of a channel. Universe and address are zero-based.
    pub fn set_channel(&mut self, channel: &DmxChannel, value: u8) -> anyhow::Result<()> {
        self.add_universe_if_absent(channel.universe);
        let universe = self.universe_mut(channel.universe).unwrap();

        universe.set_address(channel.address, value)?;
        Ok(())
    }

    pub fn apply_changes(&mut self, changes: &HashMap<DmxChannel, u8>) -> anyhow::Result<()> {
        for (channel, value) in changes.iter() {
            self.set_channel(channel, *value)?
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DmxUniverse {
    id: u16,
    addresses: [u8; 512],
}

impl DmxUniverse {
    pub fn new(id: u16) -> Self {
        DmxUniverse {
            id,
            addresses: [0; 512],
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn set_address(&mut self, address: u16, value: u8) -> anyhow::Result<()> {
        if !(0..512).contains(&address) {
            return Err(anyhow!(
                "Tried to set address {} in universe {} but it is out of range",
                address,
                self.id
            ));
        }

        self.addresses[address as usize] = value;

        Ok(())
    }

    pub fn get_address(&self, address: u16) -> anyhow::Result<u8> {
        if !(0..512).contains(&address) {
            return Err(anyhow!(
                "Tried to get address {} in universe {} but it is out of range",
                address,
                self.id
            ));
        }

        Ok(self.addresses[address as usize])
    }

    pub fn get_addresses(&self) -> &[u8; 512] {
        &self.addresses
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct DmxChannel {
    pub universe: u16,
    pub address: u16,
}

impl DmxChannel {
    pub fn new(universe: u16, address: u16) -> anyhow::Result<Self> {
        if address > 511 {
            return Err(anyhow!(
                "Invalid DMX address: {address}. Should be in range 0..=511"
            ));
        }

        Ok(DmxChannel { universe, address })
    }
}

impl FromStr for DmxChannel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 2 {
            return Err(anyhow!("Failed to parse DMX string '{s}'"));
        }

        let universe = parts[0]
            .parse()
            .map_err(|_| anyhow!("Failed to parse DMX string '{s}'"))?;
        let channel = parts[1]
            .parse()
            .map_err(|_| anyhow!("Failed to parse DMX string '{s}'"))?;

        Ok(DmxChannel {
            universe,
            address: channel,
        })
    }
}

impl Display for DmxChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{:03}", self.universe, self.address)
    }
}

#[macro_export]
macro_rules! channel {
    ($universe:expr, $address:expr) => {
        dmx::DmxChannel::new($universe, $address)
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DmxValue(u32);

impl DmxValue {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn raw_values_for_channel_resolution(&self, channel_resolution: u8) -> Vec<u8> {
        let mut bytes = self.0.to_le_bytes().to_vec();
        bytes.truncate(channel_resolution as usize);
        bytes
    }
}
