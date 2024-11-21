//! This module contains the [DmxUniverse] struct, which represents a single DMX universe.
//! A universe contains 512 channels, where each channel has a value between 0 and 255, represented as a [u8].

use crate::DmxChannel;

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
    fn test_new() {
        let universe = DmxUniverse::new();
        assert_eq!(universe.0.len(), 512);
        assert_eq!(universe.0.iter().all(|&x| x == 0), true);
    }

    #[test]
    fn test_set_channel_value() {
        let mut universe = DmxUniverse::new();
        universe.set_channel_value(DmxChannel::new(1).unwrap(), 255);
        assert_eq!(universe.0[0], 255);

        let mut universe = DmxUniverse::new();
        universe.set_channel_value(DmxChannel::new(255).unwrap(), 255);
        assert_eq!(universe.0[254], 255);
    }

    #[test]
    fn test_get_channel_value() {
        let mut universe = DmxUniverse::new();
        universe.0[0] = 255;
        assert_eq!(universe.get_channel_value(DmxChannel::new(1).unwrap()), 255);
    }
}
