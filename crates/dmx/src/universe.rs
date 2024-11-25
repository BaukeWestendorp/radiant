//! This module contains the [DmxUniverse] struct, which represents a single DMX universe.
//! A universe contains 512 channels, where each channel has a value between 0 and 255, represented as a [u8].

use std::str::FromStr;

use crate::{DmxChannel, DmxError};

/// A [DmxUniverseId] is the numerical identifier of a [DmxUniverse].
/// It is a number between 1 and 65535
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct DmxUniverseId(u16);

impl DmxUniverseId {
    /// Create a new [DmxUniverseId] with the given `id`.
    ///
    /// # Errors
    /// Returns a [DmxError::InvalidUniverseId] if the `id` is not between 1 and 65535.
    pub fn new(id: u16) -> crate::Result<Self> {
        if id > 0 {
            Ok(Self(id))
        } else {
            Err(DmxError::InvalidUniverseId(id))
        }
    }

    /// Create a new [DmxUniverseId] with the given `id`, clamped to the range 1..=65535.
    pub fn new_clamped(id: u16) -> Self {
        Self(id.clamp(1, u16::MAX))
    }

    /// Get the value of the [DmxUniverseId].
    pub fn value(&self) -> u16 {
        self.0
    }
}

impl Default for DmxUniverseId {
    fn default() -> Self {
        Self(1)
    }
}

impl std::fmt::Display for DmxUniverseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for DmxUniverseId {
    type Err = DmxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u16>() {
            Ok(id) => DmxUniverseId::new(id),
            Err(err) => Err(DmxError::ParseFailed {
                message: format!("Failed to parse universe id: {}", err),
            }),
        }
    }
}

/// A [DmxUniverse] represents a single universe in a DMX controller.
/// It contains 512 channels, where each channel has a value between 0 and 255, represented as a [u8].
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DmxUniverse(Vec<u8>);

impl DmxUniverse {
    /// Create a new, empty [DmxUniverse].
    pub fn new() -> Self {
        Self(vec![0; 512])
    }

    /// Get the raw bytes of the [DmxUniverse].
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }

    /// Set the value at a specific `channel`.
    pub fn set_channel_value(&mut self, channel: DmxChannel, value: u8) {
        self.0[channel.value() as usize - 1] = value;
    }

    /// Get the value at a specific `channel`.
    pub fn get_channel_value(&self, channel: DmxChannel) -> u8 {
        self.0[channel.value() as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_id() {
        let id = DmxUniverseId::new(1).unwrap();
        assert_eq!(id.value(), 1);
    }

    #[test]
    fn test_new_id_invalid() {
        let id = DmxUniverseId::new(0);
        assert!(id.is_err());
    }

    #[test]
    fn test_new_dmx_universe() {
        let universe = DmxUniverse::new();
        assert_eq!(universe.0.len(), 512);
        assert!(universe.0.iter().all(|&x| x == 0));
    }

    #[test]
    fn test_dmx_universe_set_channel_value() {
        let mut universe = DmxUniverse::new();
        universe.set_channel_value(DmxChannel::new(1).unwrap(), 255);
        assert_eq!(universe.0[0], 255);

        let mut universe = DmxUniverse::new();
        universe.set_channel_value(DmxChannel::new(255).unwrap(), 255);
        assert_eq!(universe.0[254], 255);
    }

    #[test]
    fn test_dmx_universe_get_channel_value() {
        let mut universe = DmxUniverse::new();
        universe.0[0] = 255;
        assert_eq!(universe.get_channel_value(DmxChannel::new(1).unwrap()), 255);
    }
}
