/// Error type for various error conditions that can occur.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid preamble size.
    #[error("Invalid preamble size: {0}")]
    InvalidPreambleSize(u16),
    /// Invalid postamble size.
    #[error("Invalid postamble size: {0}")]
    InvalidPostambleSize(u16),
    /// Invalid ACN packet identifier.
    #[error(
        "Invalid ACN packet identifier: {0:?}, expected {expected:?}",
        expected = crate::packet::ROOT_PACKET_IDENTIFIER
    )]
    InvalidAcnPacketIdentifier([u8; 12]),
    /// Invalid component ID.
    #[error("Invalid component ID: {0:?}")]
    InvalidComponentId([u8; 64]),
    /// Invalid priority.
    #[error("Invalid priority: {0}. Must be between 0 and 200.")]
    InvalidPriority(u8),
    /// Invalid source name length.
    #[error("Invalid source name length: {0}. Must be between 0 and 64.")]
    InvalidSourceNameLength(usize),

    /// [std::io::Error] wrapper.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Invalid packet.
    #[error("Invalid packet")]
    InvalidPacket,
}
