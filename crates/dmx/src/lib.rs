#![warn(missing_docs)]

//! # DMX
//! This library provides a Rust interface for working with DMX data.
//!
//! # Features
//! - [Address] validation for DMX channels 1..=512.
//! - [Universe] and [Multiverse] abstractions.
//! - Helper functions for absolute addressing.

use std::collections::HashMap;

mod error;

pub use error::Error;

/// A DMX channel.
///
/// Ensures that the channel number is valid when constructed.
/// Valid channel numbers are within the range 1..=512.
///
/// # Examples
///
/// ```
/// # use dmx::Channel;
/// // Create a valid channel
/// let valid_channel = Channel::new(100);
/// assert!(valid_channel.is_ok());
///
/// // Create invalid channels
/// let invalid_channel = Channel::new(0);
/// assert!(invalid_channel.is_err());
/// let invalid_channel = Channel::new(513);
/// assert!(invalid_channel.is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct Channel(u16);

impl Channel {
    /// The minimum valid channel number.
    pub const MIN: Self = Self(1);

    /// The maximum valid channel number.
    pub const MAX: Self = Self(512);

    /// Creates a new [Channel] within the valid DMX range 1..=512.
    ///
    /// Returns an error if the channel number is outside the valid range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dmx::Channel;
    /// let valid = Channel::new(100);
    /// assert!(valid.is_ok());
    /// let invalid = Channel::new(513);
    /// assert!(invalid.is_err());
    /// ```
    pub fn new(channel: u16) -> Result<Self, Error> {
        match channel {
            1..=512 => Ok(Self(channel)),
            other => Err(Error::InvalidChannel(other)),
        }
    }
}

impl Default for Channel {
    fn default() -> Self {
        Self::new(1).unwrap()
    }
}

impl From<Channel> for u16 {
    fn from(channel: Channel) -> Self {
        channel.0
    }
}

impl TryFrom<u16> for Channel {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl std::str::FromStr for Channel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let channel = s.parse::<u16>().map_err(|_| Error::ParseChannelFailed(s.to_string()))?;
        Self::new(channel)
    }
}

impl std::ops::Deref for Channel {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Channel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for Channel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::new(u16::deserialize(deserializer)?).map_err(::serde::de::Error::custom)
    }
}

/// Represents an 8-bit DMX value from 0-255.
///
/// # Examples
///
/// ```
/// let val = dmx::Value(128); // Create a DMX value of 128
/// let min = dmx::Value(0);   // Minimum DMX value
/// let max = dmx::Value(255); // Maximum DMX value
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Value(pub u8);

impl Value {
    /// The minimum valid DMX value.
    pub const MIN: Self = Value(0);

    /// The maximum valid DMX value.
    pub const MAX: Self = Value(255);
}

impl From<Value> for u8 {
    fn from(value: Value) -> Self {
        value.0
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value(value)
    }
}

impl std::ops::Deref for Value {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Value {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A unique DMX address composed of a [UniverseId] and a [Channel].
///
/// Each DMX address consists of two components:
/// - A universe ID 1..=65536
/// - A channel number 1..=512
///
/// The address can be created either by explicitly providing the universe and channel,
/// or by converting from an absolute address.
///
/// # Examples
///
/// ```
/// # use dmx::{Address, Channel, UniverseId};
/// // Create an address in universe 1, channel 100
/// let addr = Address::new(UniverseId::new(1).unwrap(), Channel::new(100).unwrap());
///
/// // Create from an absolute address
/// let addr = Address::from_absolute(1000).unwrap();
/// assert_eq!(addr.universe, UniverseId::new(2).unwrap());
/// assert_eq!(addr.channel, Channel::new(488).unwrap());
/// ```
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Address {
    /// The universe id for this address.
    pub universe: UniverseId,
    /// The channel for this address 1..=512.
    pub channel: Channel,
}

impl Address {
    /// Creates a new [Address] from a universe ID and channel.
    pub fn new(universe: UniverseId, channel: Channel) -> Self {
        Self { universe, channel }
    }

    /// Creates a new [Address] from an absolute address.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use dmx::{Address, Channel, UniverseId};
    /// let address = Address::from_absolute(1000).unwrap();
    /// assert_eq!(address.universe, UniverseId::new(2).unwrap());
    /// assert_eq!(address.channel, Channel::new(488).unwrap());
    /// ```
    pub fn from_absolute(absolute_address: u32) -> Result<Self, Error> {
        // Handle case where absolute_address is 0
        if absolute_address == 0 {
            return Err(Error::InvalidChannel(0));
        }

        let universe_idx = (absolute_address - 1) / 512;
        let channel_num = (absolute_address - 1) % 512 + 1;

        Ok(Self {
            universe: UniverseId(1 + universe_idx as u16),
            channel: Channel::new(channel_num as u16)?,
        })
    }

    /// Converts the [Address] to an absolute address.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use dmx::{Address, Channel, UniverseId};
    /// let address = Address::new(UniverseId::new(2).unwrap(), Channel::new(488).unwrap());
    /// assert_eq!(address.to_absolute(), 1000);
    /// ```
    pub fn to_absolute(&self) -> u32 {
        (self.universe.0 as u32 - 1) * 512 + self.channel.0 as u32
    }

    /// Returns a new [Address] with the channel offset by the specified amount.
    ///
    /// This method adds the given offset to the current channel value and ensures
    /// the resulting channel is valid (within 1..=512 range).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use dmx::{Address, Channel, UniverseId};
    /// let address = Address::new(UniverseId::new(1).unwrap(), Channel::new(500).unwrap());
    /// let new_address = address.with_channel_offset(10).unwrap();
    /// assert_eq!(new_address.channel, Channel::new(510).unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the resulting channel would be outside the valid range (1..=512).
    ///
    /// ```rust
    /// # use dmx::{Address, Channel, UniverseId};
    /// let address = Address::new(UniverseId::new(1).unwrap(), Channel::new(500).unwrap());
    /// let result = address.with_channel_offset(20); // Would be channel 520
    /// assert!(result.is_err());
    /// ```
    pub fn with_channel_offset(mut self, offset: u16) -> Result<Self, Error> {
        self.channel = Channel::new(self.channel.0 + offset)?;
        Ok(self)
    }
}

impl std::str::FromStr for Address {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 2 {
            return Err(Error::ParseAddressFailed(s.to_string()));
        }

        let universe = parts[0].parse::<UniverseId>()?;
        let channel = parts[1].parse::<Channel>()?;

        Ok(Self { universe, channel })
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.universe, self.channel)
    }
}

/// A DMX universe ID.
///
/// Must be greater than 0.
///
/// # Examples
///
/// ```
/// # use dmx::UniverseId;
/// // Valid universe ID
/// let valid_universe = UniverseId::new(1);
/// assert!(valid_universe.is_ok());
///
/// // Invalid universe ID
/// let invalid = UniverseId::new(0);
/// assert!(invalid.is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct UniverseId(u16);

impl UniverseId {
    /// The minimum valid universe ID.
    pub const MIN: Self = Self(1);

    /// The maximum valid universe ID.
    pub const MAX: Self = Self(u16::MAX);

    /// Creates a new universe ID from the given number.
    ///
    /// Universe IDs must be greater than 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dmx::UniverseId;
    /// let valid_universe = UniverseId::new(1);
    /// assert!(valid_universe.is_ok());
    ///
    /// let invalid = UniverseId::new(0);
    /// assert!(invalid.is_err());
    /// ```
    pub const fn new(id: u16) -> Result<Self, Error> {
        if id == 0 {
            return Err(Error::InvalidUniverseId(id));
        }

        Ok(Self(id))
    }
}

impl Default for UniverseId {
    fn default() -> Self {
        Self::new(1).unwrap()
    }
}

impl TryFrom<u16> for UniverseId {
    type Error = Error;

    fn try_from(id: u16) -> Result<Self, Self::Error> {
        Self::new(id)
    }
}

impl From<UniverseId> for u16 {
    fn from(universe_id: UniverseId) -> Self {
        universe_id.0
    }
}

impl std::str::FromStr for UniverseId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = u16::from_str(s).map_err(|_| Error::ParseUniverseIdFailed(s.to_string()))?;
        UniverseId::new(v)
    }
}

impl std::ops::Deref for UniverseId {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for UniverseId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for UniverseId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for UniverseId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::new(u16::deserialize(deserializer)?).map_err(::serde::de::Error::custom)
    }
}

/// A DMX universe that contains 512 [Value]s.
///
/// The universe has:
/// - A unique ID ([UniverseId])
/// - An array of 512 DMX values
///
/// # Examples
///
/// ```
/// # use dmx::{Universe, UniverseId, Value};
/// let universe = Universe::new();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Universe {
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    values: [Value; 512],
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}

impl Universe {
    /// Creates a new universe.
    ///
    /// All values are initialized to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dmx::Universe;
    /// let universe = Universe::new();
    /// ```
    pub fn new() -> Self {
        Self { values: [Value::default(); 512] }
    }

    /// Get the value for the given channel.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dmx::{Universe, Channel, Value};
    /// let universe = Universe::new();
    /// let channel = Channel::new(1).unwrap();
    /// assert_eq!(universe.get_value(&channel), Value(0));
    /// ```
    pub fn get_value(&self, channel: &Channel) -> Value {
        self.values[channel.0 as usize - 1]
    }

    /// Sets a value at a given channel.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dmx::{Universe, Channel, Value};
    /// let mut universe = Universe::new();
    ///
    /// let channel = Channel::new(1).unwrap();
    /// universe.set_value(&channel, Value(128));
    /// assert_eq!(universe.get_value(&channel), Value(128));
    /// ```
    pub fn set_value(&mut self, channel: &Channel, value: Value) {
        self.values[channel.0 as usize - 1] = value;
    }

    /// Returns an immutable reference to the values.
    ///
    /// **Note**: The indices of this array are 0-based but the channel values are 1-based.
    /// For example, channel 1 maps to index 0 in the array.
    ///
    pub fn values(&self) -> &[Value; 512] {
        &self.values
    }

    /// Returns a mutable reference to the values.
    /// **Note**: The indices of this array are 0-based but the channel values are 1-based.
    /// For example, channel 1 maps to index 0 in the array.
    ///
    pub fn values_mut(&mut self) -> &mut [Value; 512] {
        &mut self.values
    }

    /// Clears all values in the universe, setting them to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dmx::Universe;
    /// let mut universe = Universe::new();
    /// universe.clear();
    /// ```
    pub fn clear(&mut self) {
        self.values = [Value::default(); 512];
    }
}

impl From<Universe> for Vec<u8> {
    fn from(universe: Universe) -> Self {
        universe.values.into_iter().map(|v| v.0).collect()
    }
}

/// A [Multiverse] contains multiple [Universe]s.
///
/// # Examples
///
/// ```
/// # use dmx::{Multiverse, Universe, UniverseId};
/// let mut multiverse = Multiverse::new();
///
/// // Add a universe
/// let id = UniverseId::new(1).unwrap();
/// let universe = Universe::new();
/// multiverse.create_universe(id, universe);
///
/// // Remove a universe
/// let _removed_universe = multiverse.remove_universe(&id);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Multiverse {
    universes: HashMap<UniverseId, Universe>,
}

impl Multiverse {
    /// Creates a new [Multiverse] with no [Universe]s in it.
    pub fn new() -> Self {
        Self { universes: HashMap::new() }
    }

    /// Checks if a [Universe] with the given [UniverseId] exists in the [Multiverse].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dmx::{Multiverse, Universe, UniverseId};
    /// let mut multiverse = Multiverse::new();
    /// let id = UniverseId::new(1).unwrap();
    /// let universe = Universe::new();
    /// multiverse.create_universe(id, universe);
    ///
    /// assert!(multiverse.has_universe(&id));
    /// ```
    pub fn has_universe(&self, id: &UniverseId) -> bool {
        self.universes.contains_key(id)
    }

    /// Creates a [Universe] and registers it in the [Multiverse].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dmx::{Multiverse, Universe, UniverseId};
    /// let mut multiverse = Multiverse::new();
    /// multiverse.create_universe(UniverseId::new(1).unwrap(), Universe::new());
    /// ```
    pub fn create_universe(&mut self, id: UniverseId, universe: Universe) {
        self.universes.insert(id, universe);
    }

    /// Removes a [Universe] with the given [UniverseId] from the [Multiverse].
    ///
    /// Returns `Some(Universe)` if a universe for that ID was present, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dmx::{Multiverse, Universe, UniverseId};
    /// let mut multiverse = Multiverse::new();
    /// let id = UniverseId::new(1).unwrap();
    /// multiverse.create_universe(id, Universe::new());
    ///
    /// let universe = multiverse.remove_universe(&id);
    /// assert!(universe.is_some());
    /// ```
    pub fn remove_universe(&mut self, id: &UniverseId) -> Option<Universe> {
        self.universes.remove(id)
    }

    /// Sets all DMX values in every [Universe] within the [Multiverse] to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dmx::{Multiverse, Universe, UniverseId};
    /// let mut multiverse = Multiverse::new();
    /// multiverse.create_universe(UniverseId::new(1).unwrap(), Universe::new());
    /// multiverse.create_universe(UniverseId::new(2).unwrap(), Universe::new());
    ///
    /// multiverse.clear();
    ///
    /// for (_, universe) in multiverse.universes() {
    ///     assert!(universe.values().iter().all(|&value| value == 0));
    /// }
    /// ```
    pub fn clear(&mut self) {
        for universe in self.universes.values_mut() {
            universe.clear();
        }
    }

    /// Returns an immutable reference to the [Universe] with the given [UniverseId].
    ///
    /// Returns `None` if no universe exists with that ID.
    pub fn universe(&self, id: &UniverseId) -> Option<&Universe> {
        self.universes.get(id)
    }

    /// Returns an mutable reference to the [Universe] with the given [UniverseId].
    ///
    /// Returns `None` if no universe exists with that ID.
    pub fn universe_mut(&mut self, id: &UniverseId) -> Option<&mut Universe> {
        self.universes.get_mut(id)
    }

    /// Returns an iterator over a reference to every [Universe] in the [Multiverse].
    pub fn universes(&self) -> impl Iterator<Item = (&UniverseId, &Universe)> {
        self.universes.iter()
    }

    /// Sets a value at a given [Address].
    ///
    /// Creates a new universe if the target universe does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dmx::{Multiverse, Universe, UniverseId, Address, Channel, Value};
    /// let mut multiverse = Multiverse::new();
    /// let id = UniverseId::new(1).unwrap();
    /// multiverse.create_universe(id, Universe::new());
    ///
    /// let address = Address::new(id, Channel::new(1).unwrap());
    /// multiverse.set_value(&address, Value(128));
    /// ```
    pub fn set_value(&mut self, address: &Address, value: Value) {
        let universe = match self.universe_mut(&address.universe) {
            Some(universe) => universe,
            _ => {
                self.create_universe(address.universe, Universe::new());
                self.universe_mut(&address.universe).unwrap()
            }
        };

        universe.set_value(&address.channel, value);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Address, Channel, UniverseId};

    #[test]
    fn universe_id_value_in_range() {
        let universe_id = Channel::new(3);
        assert!(universe_id.is_ok());
    }

    #[test]
    fn universe_id_value_too_low() {
        let universe_id = Channel::new(0);
        assert!(universe_id.is_err());
    }

    #[test]
    fn channel_value_in_range() {
        let channel = Channel::new(300);
        assert!(channel.is_ok());
    }

    #[test]
    fn channel_value_too_high() {
        let channel = Channel::new(513);
        assert!(channel.is_err());
    }

    #[test]
    fn channel_value_too_low() {
        let channel = Channel::new(0);
        assert!(channel.is_err());
    }

    #[test]
    fn address_new_valid() {
        let address = Address::new(UniverseId::new(2).unwrap(), Channel::new(100).unwrap());
        assert_eq!(address.universe, UniverseId::new(2).unwrap());
        assert_eq!(address.channel, Channel::new(100).unwrap());
    }

    #[test]
    fn address_from_absolute_address_valid() {
        let address = Address::from_absolute(1000).unwrap();
        assert_eq!(address.universe, UniverseId::new(2).unwrap());
        assert_eq!(address.channel, Channel::new(488).unwrap());
    }

    #[test]
    fn address_sorting_order() {
        let mut addresses = vec![
            Address::new(UniverseId::new(2).unwrap(), Channel::new(50).unwrap()),
            Address::new(UniverseId::new(1).unwrap(), Channel::new(512).unwrap()),
            Address::new(UniverseId::new(1).unwrap(), Channel::new(1).unwrap()),
            Address::new(UniverseId::new(2).unwrap(), Channel::new(1).unwrap()),
            Address::new(UniverseId::new(1).unwrap(), Channel::new(250).unwrap()),
        ];
        addresses.sort();
        let expected = vec![
            Address::new(UniverseId::new(1).unwrap(), Channel::new(1).unwrap()),
            Address::new(UniverseId::new(1).unwrap(), Channel::new(250).unwrap()),
            Address::new(UniverseId::new(1).unwrap(), Channel::new(512).unwrap()),
            Address::new(UniverseId::new(2).unwrap(), Channel::new(1).unwrap()),
            Address::new(UniverseId::new(2).unwrap(), Channel::new(50).unwrap()),
        ];
        assert_eq!(addresses, expected);
    }

    #[test]
    fn address_ord_less() {
        let a = Address::new(UniverseId::new(3).unwrap(), Channel::new(100).unwrap());
        let b = Address::new(UniverseId::new(3).unwrap(), Channel::new(101).unwrap());
        let c = Address::new(UniverseId::new(4).unwrap(), Channel::new(1).unwrap());
        assert!(a < b);
        assert!(b < c);
    }
}

#[cfg(feature = "serde")]
mod serde {
    #[cfg(test)]
    mod tests {
        use crate::*;

        #[test]
        fn serialize_channel() {
            let channel = Channel::new(100).unwrap();
            let serialized = serde_json::to_string(&channel).unwrap();
            assert_eq!(serialized, "100");
        }

        #[test]
        fn deserialize_channel() {
            let channel: Channel = serde_json::from_str("100").unwrap();
            assert_eq!(channel, Channel::new(100).unwrap());
        }

        #[test]
        fn deserialize_invalid_channel() {
            let result: Result<Channel, _> = serde_json::from_str("513");
            assert!(result.is_err());
        }

        #[test]
        fn serialize_universe_id() {
            let universe_id = UniverseId::new(1).unwrap();
            let serialized = serde_json::to_string(&universe_id).unwrap();
            assert_eq!(serialized, "1");
        }

        #[test]
        fn deserialize_universe_id() {
            let universe_id: UniverseId = serde_json::from_str("1").unwrap();
            assert_eq!(universe_id, UniverseId::new(1).unwrap());
        }

        #[test]
        fn deserialize_invalid_universe_id() {
            let result: Result<Channel, _> = serde_json::from_str("0");
            assert!(result.is_err());
        }

        #[test]
        fn deserialize_universe() {
            let json = r#"{"id":1,"values":[0,0,0]}"#;
            let universe: Result<Universe, _> = serde_json::from_str(json);
            assert!(universe.is_err()); // Should fail as we need all 512 values
        }
    }
}
