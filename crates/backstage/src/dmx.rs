//! # DMX utilities.
//!
//! This module provides utilities for working with DMX.

use std::{collections::HashMap, fmt::Display, str::FromStr};

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

/// A DMX universe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DmxUniverse {
    channels: [u8; 512],
}

impl DmxUniverse {
    /// Create a new DMX universe.
    pub fn new() -> Self {
        DmxUniverse { channels: [0; 512] }
    }

    /// Get the value of a DMX channel.
    /// The address is 1-indexed.
    ///
    /// # Errors
    ///
    /// Returns an error if the address is not in the range 1..=512.
    pub fn get_channel(&self, address: u16) -> Result<u8, Error> {
        if address == 0 || address > 512 {
            return Err(Error::InvalidAddress(address));
        }

        Ok(self.channels[(address - 1) as usize])
    }

    /// Set the value of a DMX channel.
    /// The address is 1-indexed.
    ///
    /// # Errors
    ///
    /// Returns an error if the address is not in the range 1..=512.
    pub fn set_channel(&mut self, address: u16, value: u8) -> Result<(), Error> {
        if address == 0 || address > 512 {
            return Err(Error::InvalidAddress(address));
        }

        self.channels[(address - 1) as usize] = value;
        Ok(())
    }
}

/// Represents the DMX output as bytes per universe.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DmxOutput {
    universes: HashMap<u16, DmxUniverse>,
}

impl DmxOutput {
    /// Create a new DMX output.
    pub fn new() -> Self {
        DmxOutput {
            universes: HashMap::new(),
        }
    }

    /// Get the value of a channel.
    /// Returns `None` if the channel is not found.
    ///
    /// # Example
    ///
    /// ```
    /// # use backstage::dmx::{DmxChannel, DmxOutput};
    /// let mut output = DmxOutput::new();
    /// let channel = DmxChannel::new(1, 1).unwrap();
    /// assert_eq!(output.get_value(&channel), None);
    ///
    /// output.set_value(&channel, 255);
    /// assert_eq!(output.get_value(&channel), Some(255));
    /// ```
    pub fn get_value(&self, channel: &DmxChannel) -> Option<u8> {
        self.universes
            .get(&channel.universe)
            .and_then(|universe| universe.get_channel(channel.address).ok())
    }

    /// Set the value of a channel.
    /// If the universe does not exist, it will be created.
    ///
    /// # Example
    ///
    /// ```
    /// # use backstage::dmx::{DmxChannel, DmxOutput};
    /// let mut output = DmxOutput::new();
    /// let channel = DmxChannel::new(1, 1).unwrap();
    /// output.set_value(&channel, 255);
    /// assert_eq!(output.get_value(&channel), Some(255));
    /// ```
    pub fn set_value(&mut self, channel: &DmxChannel, value: u8) -> Result<(), Error> {
        if !self.universes.contains_key(&channel.universe) {
            self.universes.insert(channel.universe, DmxUniverse::new());
        }
        self.universes
            .get_mut(&channel.universe)
            .unwrap()
            .set_channel(channel.address, value)?;

        Ok(())
    }

    /// Set the values of multiple channels.
    /// If the universe does not exist, it will be created.
    /// Offsets are 0-based.
    ///
    /// # Errors
    ///
    /// This function will return an error if the length of the offsets and values does not match.
    ///
    /// # Example
    ///
    /// ```
    /// # use backstage::dmx::{DmxChannel, DmxOutput};
    /// let mut output = DmxOutput::new();
    /// let start_channel = DmxChannel::new(1, 1).unwrap();
    /// output.set_values(&start_channel, &[0, 1], &[255, 128]).unwrap();
    /// assert_eq!(output.get_value(&DmxChannel::new(1, 1).unwrap()), Some(255));
    /// assert_eq!(output.get_value(&DmxChannel::new(1, 2).unwrap()), Some(128));
    /// ```
    pub fn set_values(
        &mut self,
        start_channel: &DmxChannel,
        offsets: &[u16],
        values: &[u8],
    ) -> Result<(), Error> {
        if offsets.len() != values.len() {
            return Err(Error::LengthMismatch(offsets.len(), values.len()));
        }

        for (offset, value) in offsets.iter().zip(values.iter()) {
            let mut channel = start_channel.clone();
            channel.address += *offset;
            self.set_value(&channel, *value)?;
        }

        Ok(())
    }
}

/// An error that can occur when working with DMX.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// An invalid DMX address.
    #[error("Invalid DMX address: {0}. Should be in range 1..=512")]
    InvalidAddress(u16),
    /// Failed to parse a DMX string.
    #[error("Failed to parse DMX string '{0}'")]
    ParseError(String),
    /// Error when the lengths of two slices do not match.
    #[error("Length mismatch: {0} != {1}")]
    LengthMismatch(usize, usize),
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
