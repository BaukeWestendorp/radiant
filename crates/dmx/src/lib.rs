use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DmxUniverse {
    id: u16,
    channels: Vec<u8>,
}

impl DmxUniverse {
    pub fn new(id: u16) -> Result<Self, Error> {
        if id == 0 {
            return Err(Error::InvalidUniverseRange(id.into()));
        }

        Ok(DmxUniverse {
            id,
            channels: Vec::with_capacity(512),
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

    pub fn get_channels(&self) -> &Vec<u8> {
        &self.channels
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DmxChannel {
    pub universe: u16,
    pub address: u16,
}

impl DmxChannel {
    pub fn new(universe: u16, address: u16) -> Result<Self, Error> {
        if universe == 0 {
            return Err(Error::InvalidUniverseRange(universe.into()));
        }

        if address == 0 || address > 512 {
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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid channel string: '{0}'")]
    InvalidDmxChannelString(String),
    #[error("Invalid universe range: {0}. A universe id should be more than 0.")]
    InvalidUniverseRange(i64),
    #[error("Invalid address range: {0}. An address should be within 0 and 512.")]
    InvalidAddressRange(i64),
}
