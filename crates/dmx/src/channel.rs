//! This module contains the [DmxChannel] struct, which represents a channel in a DMX universe.
//! A channel has a value between 1 and 512.

use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use crate::DmxError;

/// A [DmxChannel] is a channel in a DMX universe, and has a value between 1 and 512.
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct DmxChannel(u16);

impl DmxChannel {
    /// Create a new [DmxChannel] with the given `value`.
    ///
    /// # Errors
    /// Returns an error if the value is not between 1 and 512.
    pub fn new(value: u16) -> crate::Result<Self> {
        if !(1..=512).contains(&value) {
            return Err(DmxError::InvalidChannel(value));
        }
        Ok(Self(value))
    }

    /// Set the value of the [DmxChannel].
    ///
    /// # Errors
    /// Returns an error if the value is not between 1 and 512.
    pub fn set_value(&mut self, value: u16) -> crate::Result<()> {
        if !(1..=512).contains(&value) {
            return Err(DmxError::InvalidChannel(value));
        }
        self.0 = value;
        Ok(())
    }

    /// Get the value of the [DmxChannel]. The value is between 1 and 512.
    pub fn value(&self) -> u16 {
        self.0
    }
}

impl FromStr for DmxChannel {
    type Err = DmxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(
            s.parse()
                .map_err(|err: ParseIntError| DmxError::ParseFailed {
                    message: err.to_string(),
                })?,
        )
    }
}

impl Display for DmxChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
