use std::collections::HashMap;
use std::{fmt, str};

pub use error::Error;

/// This module contains data types for error handling.
mod error {
    use std::fmt;

    use super::UniverseId;

    /// Error type for various error conditions that can occur during DMX operations.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Error {
        /// Error when a channel value is invalid.
        InvalidChannel(u16),
        /// Error when a universe ID is invalid (e.g. 0 or otherwise disallowed).
        InvalidUniverseId(u16),
        /// Error when a universe ID is out of the representable range after arithmetic operations.
        /// Carries the computed out-of-range value as an `i64`.
        UniverseIdOutOfRange(i64),
        /// Error when a universe with the specified ID cannot be found.
        UniverseNotFound(UniverseId),

        /// Parsing channel failed.
        ParseChannelFailed(String),
        /// Parsing universe id failed.
        ParseUniverseIdFailed(String),
        /// Parsing address failed.
        ParseAddressFailed(String),
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::InvalidChannel(ch) => {
                    write!(
                        f,
                        "channel has invalid value: '{}', but should be in the range 1..=512",
                        ch
                    )
                }
                Error::InvalidUniverseId(id) => {
                    write!(f, "universe has invalid id: '{}'. Should be greater than 0", id)
                }
                Error::UniverseIdOutOfRange(v) => {
                    write!(f, "universe id out of range: '{}'", v)
                }
                Error::UniverseNotFound(u) => {
                    write!(f, "universe with id '{}' not found", u)
                }
                Error::ParseChannelFailed(s) => {
                    write!(f, "failed to parse channel: '{}'", s)
                }
                Error::ParseUniverseIdFailed(s) => {
                    write!(f, "failed to parse universe id: '{}'", s)
                }
                Error::ParseAddressFailed(s) => {
                    write!(f, "failed to parse address: '{}'", s)
                }
            }
        }
    }

    impl std::error::Error for Error {}
}

/// A DMX channel.
///
/// Ensures that the channel number is valid when constructed.
/// Valid channel numbers are within the range `1..=512`.
///
/// # Examples
///
/// ```
/// # use theymx::Channel;
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
#[derive(serde::Serialize)]
pub struct Channel(u16);

impl Channel {
    /// Returns the inner channel number as `u16`.
    pub fn as_u16(&self) -> u16 {
        self.0
    }

    /// Returns a 0-based index for array indexing (channel 1 -> index 0).
    pub fn as_index(&self) -> usize {
        (self.0 - 1) as usize
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Channel {
    /// The minimum valid channel number.
    pub const MIN: Self = Self(1);

    /// The maximum valid channel number.
    pub const MAX: Self = Self(512);

    /// Creates a new [`Channel`] within the valid DMX range `1..=512`.
    ///
    /// Returns an error if the channel number is outside the valid range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use theymx::Channel;
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

    /// Constructor that does not validate the channel.
    ///
    /// # Panics
    ///
    /// This method will panic if the channel is 0.
    pub const fn new_unchecked(channel: u16) -> Self {
        assert!(channel > 0);
        Self(channel)
    }
}

impl<'de> serde::Deserialize<'de> for Channel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::new(u16::deserialize(deserializer)?).map_err(serde::de::Error::custom)
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

impl str::FromStr for Channel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let channel = s.parse::<u16>().map_err(|_| Error::ParseChannelFailed(s.to_string()))?;
        Self::new(channel)
    }
}

/// Represents an 8-bit DMX value from 0-255.
///
/// # Examples
///
/// ```
/// # use theymx::Value;
/// let val = Value(128); // Create a DMX value of 128
/// let min = Value(0);   // Minimum DMX value
/// let max = Value(255); // Maximum DMX value
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Value(pub u8);

impl Value {
    /// Returns the inner value as `u8`.
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u8> for Value {
    fn from(v: u8) -> Self {
        Value(v)
    }
}

impl From<Value> for u8 {
    fn from(v: Value) -> Self {
        v.0
    }
}

impl str::FromStr for Value {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = u8::from_str(s)?;
        Ok(Value(v))
    }
}

impl Value {
    /// The minimum valid DMX value.
    pub const MIN: Self = Value(0);

    /// The maximum valid DMX value.
    pub const MAX: Self = Value(255);
}

/// A unique DMX address composed of a [`UniverseId`] and a [`Channel`].
///
/// Each DMX address consists of two components:
/// - A universe ID `1..=65536`
/// - A channel number `1..=512`
///
/// The address can be created either by explicitly providing the universe and
/// channel, or by converting from an absolute address.
///
/// # Examples
///
/// ```
/// # use theymx::{Address, UniverseId, Channel};
/// // Create an address in universe 1, channel 100
/// let addr = Address::new(UniverseId::new(1).unwrap(), Channel::new(100).unwrap());
///
/// // Create from an absolute address
/// let addr = Address::from_absolute(1000).unwrap();
/// assert_eq!(addr.universe(), UniverseId::new(2).unwrap());
/// assert_eq!(addr.channel(), Channel::new(488).unwrap());
/// ```
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Address {
    /// The universe id for this address.
    universe: UniverseId,
    /// The channel for this address.
    channel: Channel,
}

impl Address {
    /// Creates a new [`Address`] from a [`UniverseId`] and a [`Channel`].
    pub fn new(universe: UniverseId, channel: Channel) -> Self {
        Self { universe, channel }
    }

    /// Access the universe of the address.
    pub fn universe(&self) -> UniverseId {
        self.universe
    }

    /// Access the channel of the address.
    pub fn channel(&self) -> Channel {
        self.channel
    }

    /// Creates a new [`Address`] from an absolute address.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use theymx::{Address, UniverseId, Channel};
    /// let address = Address::from_absolute(1000).unwrap();
    /// assert_eq!(address.universe(), UniverseId::new(2).unwrap());
    /// assert_eq!(address.channel(), Channel::new(488).unwrap());
    /// ```
    pub fn from_absolute(absolute_address: u32) -> Result<Self, Error> {
        // Handle case where absolute_address is 0
        if absolute_address == 0 {
            return Err(Error::InvalidChannel(0));
        }

        let universe_idx = (absolute_address - 1) / 512;
        let channel_num = (absolute_address - 1) % 512 + 1;

        // Compute universe number and check it fits in u16
        let universe_num = 1_u64
            .checked_add(universe_idx as u64)
            .ok_or(Error::UniverseIdOutOfRange(universe_idx as i64))?;

        if universe_num == 0 || universe_num > u16::MAX as u64 {
            return Err(Error::UniverseIdOutOfRange(universe_num as i64));
        }

        Ok(Self {
            universe: UniverseId::new(universe_num as u16)?,
            channel: Channel::new(channel_num as u16)?,
        })
    }

    /// Converts the [`Address`] to an absolute address.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use theymx::{Address, UniverseId, Channel};
    /// let address = Address::new(UniverseId::new(2).unwrap(), Channel::new(488).unwrap());
    /// assert_eq!(address.to_absolute(), 1000);
    /// ```
    pub fn to_absolute(&self) -> u32 {
        (self.universe.0 as u32 - 1) * 512 + self.channel.0 as u32
    }

    /// Returns a new [`Address`] with the channel offset by the specified amount.
    ///
    /// This method adds the given signed offset to the current channel value.
    /// Positive offsets move forwards across channels/universes, negative
    /// offsets move backwards. If the resulting channel goes past the `1..=512`
    /// range it moves into the next (or previous) universe(s) accordingly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use theymx::{Address, UniverseId, Channel};
    /// let address = Address::new(UniverseId::new(1).unwrap(), Channel::new(500).unwrap());
    /// let new_address = address.with_channel_offset(10).unwrap();
    /// assert_eq!(new_address.channel(), Channel::new(510).unwrap());
    ///
    /// // Moves to next universe
    /// let address = Address::new(UniverseId::new(1).unwrap(), Channel::new(510).unwrap());
    /// let new_address = address.with_channel_offset(5).unwrap();
    /// assert_eq!(new_address.universe(), UniverseId::new(2).unwrap());
    /// assert_eq!(new_address.channel(), Channel::new(3).unwrap());
    ///
    /// // Negative offset moving to previous universe
    /// let address = Address::new(UniverseId::new(2).unwrap(), Channel::new(3).unwrap());
    /// let new_address = address.with_channel_offset(-5).unwrap();
    /// assert_eq!(new_address.universe(), UniverseId::new(1).unwrap());
    /// assert_eq!(new_address.channel(), Channel::new(510).unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if applying the offset would produce a universe ID
    /// outside the valid range (less than 1 or greater than u16::MAX), or if
    /// for some reason the computed channel is invalid.
    pub fn with_channel_offset(self, offset: i32) -> Result<Self, Error> {
        let current_abs = (self.universe.0 as i64 - 1) * 512 + (self.channel.0 as i64 - 1);
        let total = current_abs + offset as i64;

        let universe_idx = total.div_euclid(512); // may be negative
        let new_channel_zero = total.rem_euclid(512) as u16; // 0..=511

        let target_universe_id = 1 + universe_idx;

        // Validate universe id explicitly; when invalid return a clear out-of-range error.
        if target_universe_id < 1 || target_universe_id > u16::MAX as i64 {
            return Err(Error::UniverseIdOutOfRange(target_universe_id));
        }

        let new_channel = new_channel_zero + 1; // 1..=512
        let target_universe_id = target_universe_id as u16;

        let channel = Channel::new(new_channel)?;
        let universe = UniverseId::new(target_universe_id)?;

        Ok(Self { universe, channel })
    }
}

impl str::FromStr for Address {
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

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.universe(), self.channel())
    }
}

/// A DMX universe ID.
///
/// Must be greater than 0.
///
/// # Examples
///
/// ```
/// # use theymx::UniverseId;
/// // Valid universe ID
/// let valid_universe = UniverseId::new(1);
/// assert!(valid_universe.is_ok());
///
/// // Invalid universe ID
/// let invalid = UniverseId::new(0);
/// assert!(invalid.is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct UniverseId(u16);

impl UniverseId {
    /// Return the inner `u16` id.
    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

impl std::fmt::Display for UniverseId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
    /// let valid_universe = theymx::UniverseId::new(1);
    /// assert!(valid_universe.is_ok());
    ///
    /// let invalid = theymx::UniverseId::new(0);
    /// assert!(invalid.is_err());
    /// ```
    pub const fn new(id: u16) -> Result<Self, Error> {
        if id == 0 {
            return Err(Error::InvalidUniverseId(id));
        }

        Ok(Self(id))
    }

    /// Constructor that does not validate the id.
    ///
    /// # Panics
    ///
    ///
    pub const fn new_unchecked(id: u16) -> Self {
        assert!(id > 0 && id <= 512);
        Self(id)
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

impl str::FromStr for UniverseId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = u16::from_str(s).map_err(|_| Error::ParseUniverseIdFailed(s.to_string()))?;
        UniverseId::new(v)
    }
}

/// A DMX universe that contains 512 [`Value`]s.
///
/// The universe has:
/// - A unique ID ([UniverseId])
/// - An array of 512 DMX values
///
/// # Examples
///
/// ```
/// # use theymx::Universe;
/// let universe = Universe::new();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Universe {
    #[serde(with = "serde_big_array::BigArray")]
    values: [Value; 512],
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}

impl Universe {
    /// Creates a new [`Universe`].
    ///
    /// All [`Value`]s are initialized to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let universe = theymx::Universe::new();
    /// ```
    pub fn new() -> Self {
        Self { values: [Value::default(); 512] }
    }

    /// Get the [`Value`] for the given [`Channel`].
    ///
    /// # Examples
    ///
    /// ```
    /// let universe = theymx::Universe::new();
    /// let channel = theymx::Channel::new(1).unwrap();
    /// assert_eq!(universe.value(&channel), theymx::Value(0));
    /// ```
    pub fn value(&self, channel: &Channel) -> Value {
        self.values[channel.0 as usize - 1]
    }

    /// Sets a [`Value`] at a given [`Channel`].
    ///
    /// # Examples
    ///
    /// ```
    /// let mut universe = theymx::Universe::new();
    ///
    /// let channel = theymx::Channel::new(1).unwrap();
    /// universe.set_value(&channel, theymx::Value(128));
    /// assert_eq!(universe.value(&channel), theymx::Value(128));
    /// ```
    pub fn set_value(&mut self, channel: &Channel, value: Value) {
        self.values[channel.0 as usize - 1] = value;
    }

    /// Returns an immutable reference to the [`Value`]s.
    ///
    /// **Note**: The indices of this array are 0-based but the channel values
    /// are 1-based. For example, channel 1 maps to index 0 in the array.
    pub fn values(&self) -> &[Value; 512] {
        &self.values
    }

    /// Returns a mutable reference to the [`Value`]s.
    /// **Note**: The indices of this array are 0-based but the channel values
    /// are 1-based. For example, channel 1 maps to index 0 in the array.
    pub fn values_mut(&mut self) -> &mut [Value; 512] {
        &mut self.values
    }

    /// Clears all values, setting them to [`Value::MIN`] (0).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut universe = theymx::Universe::new();
    /// universe.clear();
    /// ```
    pub fn clear(&mut self) {
        self.values = [Value::MIN; 512];
    }
}

impl From<Universe> for Vec<u8> {
    fn from(universe: Universe) -> Self {
        universe.values.into_iter().map(|v| v.0).collect()
    }
}

/// Contains multiple [`Universe`]s.
///
/// # Examples
///
/// ```
/// let mut multiverse = theymx::Multiverse::new();
///
/// // Add a universe
/// let id = theymx::UniverseId::new(1).unwrap();
/// let universe = theymx::Universe::new();
/// multiverse.create_universe(id, universe);
///
/// // Remove a universe
/// let _removed_universe = multiverse.remove_universe(&id);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Multiverse {
    universes: HashMap<UniverseId, Universe>,
}

impl Multiverse {
    /// Creates a new [`Multiverse`] with no [`Universe`]s in it.
    pub fn new() -> Self {
        Self { universes: HashMap::new() }
    }

    /// Checks if a [`Universe`] with the given [`UniverseId`] exists in the
    /// [`Multiverse`].
    ///
    /// # Examples
    ///
    /// ```
    /// let mut multiverse = theymx::Multiverse::new();
    /// let id = theymx::UniverseId::new(1).unwrap();
    /// let universe = theymx::Universe::new();
    /// multiverse.create_universe(id, universe);
    ///
    /// assert!(multiverse.has_universe(&id));
    /// ```
    pub fn has_universe(&self, id: &UniverseId) -> bool {
        self.universes.contains_key(id)
    }

    /// Creates a [`Universe`] and registers it in the [`Multiverse`].
    ///
    /// # Examples
    ///
    /// ```
    /// let mut multiverse = theymx::Multiverse::new();
    /// multiverse.create_universe(theymx::UniverseId::new(1).unwrap(), theymx::Universe::new());
    /// ```
    pub fn create_universe(&mut self, id: UniverseId, universe: Universe) {
        self.universes.insert(id, universe);
    }

    /// Removes a [`Universe`] with the given [`UniverseId`] from the [`Multiverse`].
    ///
    /// Returns `Some(Universe)` if a universe for that ID was present, `None`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut multiverse = theymx::Multiverse::new();
    /// let id = theymx::UniverseId::new(1).unwrap();
    /// multiverse.create_universe(id, theymx::Universe::new());
    ///
    /// let universe = multiverse.remove_universe(&id);
    /// assert!(universe.is_some());
    /// ```
    pub fn remove_universe(&mut self, id: &UniverseId) -> Option<Universe> {
        self.universes.remove(id)
    }

    /// Sets all DMX values in every [`Universe`] within the [`Multiverse`] to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut multiverse = theymx::Multiverse::new();
    /// multiverse.create_universe(theymx::UniverseId::new(1).unwrap(), theymx::Universe::new());
    /// multiverse.create_universe(theymx::UniverseId::new(2).unwrap(), theymx::Universe::new());
    ///
    /// multiverse.clear();
    ///
    /// for (_, universe) in multiverse.universes() {
    ///     assert!(universe.values().iter().all(|&value| value == theymx::Value(0)));
    /// }
    /// ```
    pub fn clear(&mut self) {
        for universe in self.universes.values_mut() {
            universe.clear();
        }
    }

    /// Returns an immutable reference to the [`Universe`] with the given
    /// [`UniverseId`].
    ///
    /// Returns `None` if no universe exists with that ID.
    pub fn universe(&self, id: &UniverseId) -> Option<&Universe> {
        self.universes.get(id)
    }

    /// Returns an mutable reference to the [`Universe`] with the given
    /// [`UniverseId`].
    ///
    /// Returns `None` if no universe exists with that ID.
    pub fn universe_mut(&mut self, id: &UniverseId) -> Option<&mut Universe> {
        self.universes.get_mut(id)
    }

    /// Returns an iterator over a reference to every [`Universe`] in the
    /// [`Multiverse`].
    pub fn universes(&self) -> impl Iterator<Item = (&UniverseId, &Universe)> {
        self.universes.iter()
    }

    /// Sets a value at a given [`Address`].
    ///
    /// Creates a new universe if the target universe does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut multiverse = theymx::Multiverse::new();
    /// let id = theymx::UniverseId::new(1).unwrap();
    /// multiverse.create_universe(id, theymx::Universe::new());
    ///
    /// let address = theymx::Address::new(id, theymx::Channel::new(1).unwrap());
    /// multiverse.set_value(&address, theymx::Value(128));
    /// ```
    pub fn set_value(&mut self, address: &Address, value: Value) {
        let id = address.universe();
        let universe = match self.universe_mut(&id) {
            Some(universe) => universe,
            _ => {
                self.create_universe(id, Universe::new());
                self.universe_mut(&id).unwrap()
            }
        };

        let ch = address.channel();
        universe.set_value(&ch, value);
    }

    /// Gets a value at a given [`Address`].
    ///
    /// Returns `None` if the universe doesn't exist.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut multiverse = theymx::Multiverse::new();
    /// let id = theymx::UniverseId::new(1).unwrap();
    /// multiverse.create_universe(id, theymx::Universe::new());
    ///
    /// let address = theymx::Address::new(id, theymx::Channel::new(1).unwrap());
    /// multiverse.set_value(&address, theymx::Value(128));
    ///
    /// assert_eq!(multiverse.value(&address), Some(theymx::Value(128)));
    /// ```
    pub fn value(&self, address: &Address) -> Option<Value> {
        let id = address.universe();
        let ch = address.channel();
        let universe = self.universe(&id)?;
        Some(universe.value(&ch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn universe_id_value_in_range() {
        let universe_id = UniverseId::new(3);
        assert!(universe_id.is_ok());
    }

    #[test]
    fn universe_id_value_too_low() {
        let universe_id = UniverseId::new(0);
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
        assert_eq!(address.universe(), UniverseId::new(2).unwrap());
        assert_eq!(address.channel(), Channel::new(100).unwrap());
    }

    #[test]
    fn address_from_absolute_address_valid() {
        let address = Address::from_absolute(1000).unwrap();
        assert_eq!(address.universe(), UniverseId::new(2).unwrap());
        assert_eq!(address.channel(), Channel::new(488).unwrap());
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
