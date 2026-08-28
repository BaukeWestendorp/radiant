#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to initialize the MIDI input system: {0}")]
    MidirInit(#[from] midir::InitError),
    #[error("Failed to get the name of the MIDI port: {0}")]
    MidirPortInfo(#[from] midir::PortInfoError),
    #[error("Failed to connect to the MIDI input port: {0}")]
    MidirInputConnectError(#[from] midir::ConnectError<midir::MidiInput>),
    #[error("Empty MIDI message buffer.")]
    EmptyMidiMessageBuffer,
    #[error("Incomplete MIDI message.")]
    IncompleteMidiMessage,
    #[error("Invalid data byte in MIDI message: {0}")]
    InvalidDataByte(u8),
    #[error("Invalid status byte in MIDI message: {0}")]
    InvalidStatusByte(u8),
    #[error("Invalid MIDI value")]
    InvalidMidiValue,
}

pub type Result<T> = std::result::Result<T, Error>;
