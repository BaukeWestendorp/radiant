//! # Error handling for DMX operations.

/// The error type for DMX operations.
#[derive(Debug, thiserror::Error)]
pub enum DmxError {
    /// The channel is invalid.
    #[error("Invalid DMX channel: {0}. The channel must be between 1 and 512.")]
    InvalidChannel(u16),
    /// Failed to parse a value.
    #[error("Failed to parse: {message}")]
    ParseFailed {
        /// The error message.
        message: String,
    },
}

/// The result type for DMX operations.
pub type Result<T> = std::result::Result<T, DmxError>;
