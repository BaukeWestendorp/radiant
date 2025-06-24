use super::UniverseId;

/// Error type for various error conditions that can occur during DMX operations.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Error when a channel value is invalid
    #[error("channel has invalid value: '{0}', but should be in the range 1..=512")]
    InvalidChannel(u16),
    /// Error when a universe ID is invalid
    #[error("universe has invalid id: '{0}'. Should be greater than 1")]
    InvalidUniverseId(u16),
    /// Error when a universe with the specified ID cannot be found
    #[error("universe with id '{0}' not found")]
    UniverseNotFound(UniverseId),

    /// Parsing channel failed.
    #[error("failed to parse channel: '{0}'")]
    ParseChannelFailed(String),
    /// Parsing universe id failed.
    #[error("failed to parse universe id: '{0}'")]
    ParseUniverseIdFailed(String),
    /// Parsing address failed.
    #[error("failed to parse address: '{0}'")]
    ParseAddressFailed(String),
}
