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
                write!(f, "channel has invalid value: '{}', but should be in the range 1..=512", ch)
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
