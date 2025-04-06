#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid preamble size: {0}")]
    InvalidPreambleSize(u16),
    #[error("Invalid postamble size: {0}")]
    InvalidPostambleSize(u16),
    #[error(
        "Invalid ACN packet identifier: {0:?}, expected {expected:?}",
        expected = crate::packet::ACN_PACKET_IDENTIFIER
    )]
    InvalidAcnPacketIdentifier([u8; 12]),
    #[error("Invalid component ID: {0:?}")]
    InvalidComponentId([u8; 64]),
    #[error("Invalid priority: {0}. Must be between 0 and 200.")]
    InvalidPriority(u8),
    #[error("Invalid source name length: {0}. Must be between 0 and 64.")]
    InvalidSourceNameLength(usize),
}
