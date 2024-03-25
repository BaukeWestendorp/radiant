use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct DmxOutput(HashMap<u16, DmxUniverse>);

impl DmxOutput {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn universes(&self) -> Values<'_, u16, DmxUniverse> {
        self.0.values()
    }

    pub fn add_universe_if_absent(&mut self, id: u16) -> Result<(), Error> {
        if !self.0.contains_key(&id) {
            self.0.insert(id, DmxUniverse::new(id)?);
        }

        Ok(())
    }

    /// Removes a universe from the output. `id` is zero-based.
    pub fn remove_universe(&mut self, id: u16) {
        self.0.remove(&id);
    }

    /// Sets the value at a channel. Universe and address are zero-based.
    pub fn set_channel(&mut self, channel: &DmxChannel, value: u8) -> Result<(), Error> {
        if !self.0.contains_key(&channel.universe) {
            self.0
                .insert(channel.universe, DmxUniverse::new(channel.universe)?);
        }

        if let Some(channel_value) = self
            .0
            .get_mut(&channel.universe)
            .unwrap()
            .channels
            .get_mut(channel.address as usize)
        {
            *channel_value = value
        } else {
            return Err(Error::ChannelNotFound(*channel));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DmxUniverse {
    id: u16,
    channels: [u8; 512],
}

impl DmxUniverse {
    pub fn new(id: u16) -> Result<Self, Error> {
        Ok(DmxUniverse {
            id,
            channels: [0; 512],
        })
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn set_channel(&mut self, channel: u16, value: u8) {
        if (0..512).contains(&channel) {
            log::warn!(
                "Tried to set channel {} in universe {} but it is out of range",
                channel,
                self.id
            );
            return;
        }

        self.channels[channel as usize] = value;
    }

    pub fn get_channel(&self, channel: u16) -> Option<u8> {
        if (0..512).contains(&channel) {
            log::warn!(
                "Tried to get channel {} in universe {} but it is out of range",
                channel,
                self.id
            );
            return None;
        }

        Some(self.channels[channel as usize])
    }

    pub fn get_channels(&self) -> &[u8; 512] {
        &self.channels
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DmxChannel {
    pub universe: u16,
    pub address: u16,
}

impl DmxChannel {
    pub fn new(universe: u16, address: u16) -> Result<Self, Error> {
        if address > 511 {
            return Err(Error::InvalidAddressRange(address.into()));
        }

        Ok(DmxChannel { universe, address })
    }
}

impl FromStr for DmxChannel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 2 {
            return Err(Error::InvalidDmxChannelString(s.to_string()));
        }

        let universe = parts[0]
            .parse()
            .map_err(|_| Error::InvalidDmxChannelString(s.to_string()))?;
        let channel = parts[1]
            .parse()
            .map_err(|_| Error::InvalidDmxChannelString(s.to_string()))?;

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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid channel string: '{0}'")]
    InvalidDmxChannelString(String),
    #[error("Invalid universe range: {0}. A universe id should be more than 0.")]
    InvalidUniverseRange(i64),
    #[error("Invalid address range: {0}. An address should be within 0 and 512.")]
    InvalidAddressRange(i64),
    #[error("Channel not found: {0}")]
    ChannelNotFound(DmxChannel),
}
