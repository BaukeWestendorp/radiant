/// Error type for various error conditions that can occur.
#[derive(Debug, thiserror::Error)]
pub enum PacketError {
    /// Invalid packet.
    #[error("Invalid packet")]
    InvalidPacket,

    /// Invalid preamble size.
    #[error("Invalid preamble size in preamble: {0:4x?}")]
    InvalidPreamblePreambleSize(u16),
    /// Invalid postamble size.
    #[error("Invalid postamble size in preamble: {0:4x?}")]
    InvalidPreamblePostambleSize(u16),
    /// Invalid ACN packet identifier.
    #[error("Invalid ACN packet identifier in preamble: {0:?}")]
    InvalidPreambleAcnPacketIdentifier(Vec<u8>),

    /// Invalid Root Layer Size
    #[error("Invalid Root Layer Size: {0}")]
    InvalidRootLayerSize(usize),
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
    #[error("Invalid root vector: {0:2x?}")]
    InvalidRootLayerVector(Vec<u8>),
    /// Invalid framing vector.
    #[error("Invalid framing vector: {0:2x?}")]
    InvalidFramingLayerVector(Vec<u8>),
    /// Invalid DMP Layer Property vector.
    #[error("Invalid DMP Layer vector: {0:2x?}")]
    InvalidDmpLayerVector(Vec<u8>),
    /// Invalid Universe Discovery Vector.
    #[error("Invalid Universe List Vector: {0:2x?}")]
    InvalidUniverseDiscoveryLayerVector(Vec<u8>),

    /// Invalid DMP address type.
    #[error("Invalid DMP address type: {0:2x?}")]
    InvalidDmpAddressType(u8),
    /// Invalid DMP first property address.
    #[error("Invalid DMP first property address: {0:4x?}")]
    InvalidDmpFirstPropertyAddress(u16),
    /// Invalid DMP address increment.
    #[error("Invalid DMP address increment: {0:4x?}")]
    InvalidDmpAddressIncrement(u16),
    /// Invalid length.
    #[error("Invalid length: {0}")]
    InvalidLength(usize),
}
