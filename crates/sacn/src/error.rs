/// Error type for various error conditions that can occur.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid packet.
    #[error("Invalid packet")]
    InvalidPacket,

    /// Invalid preamble size.
    #[error("Invalid preamble size: {0:4x?}")]
    InvalidPreambleSize(u16),
    /// Invalid postamble size.
    #[error("Invalid postamble size: {0:4x?}")]
    InvalidPostambleSize(u16),
    /// Invalid ACN packet identifier.
    #[error("Invalid ACN packet identifier")]
    InvalidAcnPacketIdentifier,
    /// Invalid component ID.
    #[error("Invalid component ID")]
    InvalidComponentId,
    /// Invalid priority.
    #[error("Invalid priority: {0}. Must be between 0 and 200.")]
    InvalidPriority(u8),
    /// Invalid source name length.
    #[error("Invalid source name length: {0}. Must be between 0 and 64.")]
    InvalidSourceNameLength(usize),

    /// Invalid root vector.
    #[error("Invalid root vector: {0:8x?}")]
    InvalidRootVector(u32),
    /// Invalid framing vector.
    #[error("Invalid framing vector: {0:8x?}")]
    InvalidFramingVector(u32),
    /// Invalid DMP Set Property vector.
    #[error("Invalid DMP Set Property vector: {0:2x?}")]
    InvalidDmpSetPropertyVector(u8),
    /// Invalid Universe List Vector.
    #[error("Invalid Universe List Vector: {0:8x?}")]
    InvalidUniverseDiscoveryUniverseListVector(u32),

    /// Invalid DMP address type.
    #[error("Invalid DMP address type: {0:2x?}")]
    InvalidDmpAddressType(u8),
    /// Invalid DMP first property address.
    #[error("Invalid DMP first property address: {0:4x?}")]
    InvalidDmpFirstPropertyAddress(u16),
    /// Invalid DMP address increment.
    #[error("Invalid DMP address increment: {0:4x?}")]
    InvalidDmpAddressIncrement(u16),

    /// [std::io::Error] wrapper.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
