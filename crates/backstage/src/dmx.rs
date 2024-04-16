//! # DMX utilities.
//!
//! This module provides utilities for working with DMX.

use std::{fmt::Display, str::FromStr};

/// A DMX channel.
/// The universe and address are 1-indexed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DmxChannel {
    /// The DMX universe, 1-indexed.
    pub universe: u16,
    /// The DMX address, 1-indexed.
    pub address: u16,
}

impl DmxChannel {
    /// Create a new DMX channel.
    /// The universe and address are 1-indexed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use backstage::dmx::DmxChannel;
    /// let channel = DmxChannel::new(1, 1).unwrap();
    /// assert_eq!(channel.universe, 1);
    /// assert_eq!(channel.address, 1);
    /// ```
    ///
    /// ```
    /// # use backstage::dmx::DmxChannel;
    /// let channel = DmxChannel::new(1, 513);
    /// assert!(channel.is_err());
    /// ```
    ///
    /// ```
    /// # use backstage::dmx::DmxChannel;
    /// let channel = DmxChannel::new(0, 1);
    /// assert!(channel.is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the address is greater than 512.
    pub fn new(universe: u16, address: u16) -> Result<Self, Error> {
        if address > 512 || universe == 0 {
            return Err(Error::InvalidAddress(address));
        }

        Ok(DmxChannel { universe, address })
    }
}

impl FromStr for DmxChannel {
    type Err = Error;

    /// Parse a DMX channel from a string.
    /// The string should be in the format "address.universe", where universe and address are
    /// 1-indexed.
    /// For example, "1.001" represents the first channel in the first universe.
    ///
    /// # Examples
    ///
    /// ```
    /// # use backstage::dmx::DmxChannel;
    /// let channel: DmxChannel = "1.001".parse().unwrap();
    /// assert_eq!(channel.universe, 1);
    /// assert_eq!(channel.address, 1);
    /// ```
    ///
    /// ```
    /// # use backstage::dmx::DmxChannel;
    /// let channel: Result<DmxChannel, _> = "1.513".parse();
    /// assert!(channel.is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not in the correct format or if the universe or address
    /// are not valid.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 2 {
            return Err(Error::ParseError(s.to_string()));
        }

        let universe = parts[0]
            .parse()
            .map_err(|_| Error::ParseError(s.to_string()))?;
        let address = parts[1]
            .parse()
            .map_err(|_| Error::ParseError(s.to_string()))?;

        DmxChannel::new(universe, address)
    }
}

impl Display for DmxChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{:03}", self.universe, self.address)
    }
}

impl<'de> serde::Deserialize<'de> for DmxChannel {
    fn deserialize<D>(deserializer: D) -> Result<DmxChannel, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let values: [u16; 2] = serde::Deserialize::deserialize(deserializer)?;
        DmxChannel::new(values[0], values[1]).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for DmxChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        [self.universe, self.address].serialize(serializer)
    }
}

/// An error that can occur when working with DMX.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An invalid DMX address.
    #[error("Invalid DMX address: {0}. Should be in range 1..=512")]
    InvalidAddress(u16),
    /// Failed to parse a DMX string.
    #[error("Failed to parse DMX string '{0}'")]
    ParseError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dmx_value_serialization() {
        let channel = DmxChannel::new(1, 1).unwrap();
        let serialized = serde_json::to_string(&channel).unwrap();
        assert_eq!(serialized, "[1,1]");

        let deserialized: DmxChannel = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, channel);
    }

    #[test]
    fn test_dmx_value_deserialization() {
        let deserialized: DmxChannel = serde_json::from_str("[1,1]").unwrap();
        assert_eq!(deserialized, DmxChannel::new(1, 1).unwrap());
    }
}
