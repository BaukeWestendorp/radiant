use crate::Opcode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Network Error: {0}")]
    Network(String),
    #[error("Net must be between 0..=127")]
    InvalidNetId,
    #[error("Sub-Net must be between 0..=15")]
    InvalidSubNetId,
    #[error("Universe must be between 0..=15")]
    InvalidUniverseId,
    #[error("Port address must be a 15-bit number between 0..=32767")]
    InvalidPortAddress,
    #[error("Channel index must be between 0..=511")]
    ChannelOutOfBounds,
    #[error("Universe index must be between 0..=1023")]
    UniverseOutOfBounds,
    #[error("Invalid packet length: {0}")]
    InvalidPacketLength(usize),
    #[error("Invalid packet ID")]
    InvalidPacketId,
    #[error("Invalid opcode: {0}")]
    InvalidOpcode(u16),
    #[error("Invalid node report: {0}")]
    InvalidNodeReport(u16),
    #[error("Invalid style code: {0}")]
    InvalidStyleCode(u8),
    #[error("The provided string is too long.")]
    StringTooLong,

    // FIXME: Remove this once everything has been implemented.
    #[error("Received an unimplemented packet.")]
    UnimplementedOpcode(Opcode),
}

pub type Result<T> = std::result::Result<T, Error>;
