use crate::Opcode;

#[derive(Debug, thiserror::Error)]
#[repr(C)]
pub enum Error {
    /// IO Error: {0}
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    /// Network Error: {0}
    #[error("Network Error: {0}")]
    Network(String),

    /// Net must be between 0..=127
    #[error("Net must be between 0..=127")]
    InvalidNetId,

    /// Sub-Net must be between 0..=15
    #[error("Sub-Net must be between 0..=15")]
    InvalidSubNetId,

    /// Universe must be between 0..=15
    #[error("Universe must be between 0..=15")]
    InvalidUniverseId,

    /// Port address must be a 15-bit number between 0..=32767
    #[error("Port address must be a 15-bit number between 0..=32767")]
    InvalidPortAddress,

    /// Channel index must be between 0..=511
    #[error("Channel index must be between 0..=511")]
    ChannelOutOfBounds,

    /// Universe index must be between 0..=1023
    #[error("Universe index must be between 0..=1023")]
    UniverseOutOfBounds,

    /// Invalid packet length: {0}
    #[error("Invalid packet length: {0}")]
    InvalidPacketLength(usize),

    /// Invalid packet ID
    #[error("Invalid packet ID")]
    InvalidPacketId,

    /// Invalid opcode: {0}
    #[error("Invalid opcode: {0}")]
    InvalidOpcode(u16),

    /// Invalid node report: {0}
    #[error("Invalid node report: {0}")]
    InvalidNodeReport(u16),

    /// Invalid style code: {0}
    #[error("Invalid style code: {0}")]
    InvalidStyleCode(u8),

    /// The provided string is too long.
    #[error("The provided string is too long.")]
    StringTooLong,

    // FIXME: Remove this once everything has been implemented.
    /// Received an unimplemented packet.
    #[error("Received an unimplemented packet.")]
    UnimplementedOpcode(Opcode),
}

pub type Result<T> = std::result::Result<T, Error>;
