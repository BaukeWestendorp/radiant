//! This module contains the [DmxAddress] struct, which represents a DMX address.
//! A DMX address consists of a universe and a channel where the universe is a [u16] value and the channel is a value between 1 and 512.

use crate::{DmxChannel, DmxError};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// A [DmxAddress] is the starting point for the channels a device listens to.
/// It has a `universe` and a `channel`, where the `universe` is a [u16] value and the `channel`
/// is a value between 1 and 512, represented as a [DmxChannel].
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DmxAddress {
    /// The universe of the DMX address.
    pub universe: u16,
    /// The channel of the DMX address.
    pub channel: DmxChannel,
}

impl DmxAddress {
    /// Create a new [DmxAddress] with the given `universe` and `channel`.
    pub fn new(universe: u16, channel: DmxChannel) -> Self {
        Self { universe, channel }
    }

    /// Get the absolute address of the DMX address.
    /// The absolute address is calculated as `universe * 512 + channel`.
    ///
    /// # Example
    /// ```
    /// use dmx::DmxAddress;
    /// use dmx::DmxChannel;
    ///
    /// let address = DmxAddress::new(2, DmxChannel::new(6).unwrap());
    /// assert_eq!(address.absolute_address(), 1030);
    /// ```
    pub fn absolute_address(&self) -> u32 {
        (self.universe * 512 + self.channel.value()) as u32
    }
}

impl Default for DmxAddress {
    fn default() -> Self {
        Self {
            universe: 1,
            channel: DmxChannel::default(),
        }
    }
}

impl FromStr for DmxAddress {
    type Err = DmxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 2 {
            return Err(DmxError::ParseFailed {
                message: "Input must contain exactly one dot ('.')".to_string(),
            });
        }

        let universe = parts[0]
            .parse::<u16>()
            .map_err(|err| DmxError::ParseFailed {
                message: err.to_string(),
            })?;
        let channel = parts[1]
            .parse::<u16>()
            .map_err(|err| DmxError::ParseFailed {
                message: err.to_string(),
            })?;

        Ok(DmxAddress::new(universe, DmxChannel::new(channel)?))
    }
}

impl Display for DmxAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.universe, self.channel)
    }
}

impl<'de> serde::Deserialize<'de> for DmxAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for DmxAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dmx_address_from_str() {
        let address = DmxAddress::new(2, DmxChannel::new(6).unwrap());
        assert_eq!("2.6".parse::<DmxAddress>().unwrap(), address);
    }

    #[test]
    fn test_dmx_address_from_str_invalid() {
        assert!("2".parse::<DmxAddress>().is_err());
        assert!("2.6.7".parse::<DmxAddress>().is_err());
        assert!("2.6.7".parse::<DmxAddress>().is_err());
        assert!("2.6.7".parse::<DmxAddress>().is_err());
    }

    #[test]
    fn test_dmx_address_display() {
        let address = DmxAddress::new(2, DmxChannel::new(6).unwrap());
        assert_eq!(address.to_string(), "2.6");
    }
}
