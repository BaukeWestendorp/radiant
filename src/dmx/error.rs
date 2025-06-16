use super::UniverseId;

/// Error type for various error conditions that can occur during DMX operations.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Error when a channel value is invalid
    #[error("Channel has invalid value: {0}. Should be in the range 1..=512")]
    InvalidChannel(u16),
    /// Error when a universe ID is invalid
    #[error("Universe has invalid id: {0}. Should be greater than 1")]
    InvalidUniverseId(u16),
    /// Error when a universe with the specified ID cannot be found
    #[error("Universe with id {0} not found")]
    UniverseNotFound(UniverseId),

    /// Parsing channel failed.
    #[error("Failed to parse channel: '{0}'")]
    ParseChannelFailed(String),
    /// Parsing universe id failed.
    #[error("Failed to parse universe id: '{0}'")]
    ParseUniverseIdFailed(String),
    /// Parsing address failed.
    #[error("Failed to parse address: '{0}'")]
    ParseAddressFailed(String),
}
