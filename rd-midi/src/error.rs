use facet_error as error;

#[derive(Debug)]
#[derive(facet::Facet)]
#[facet(derive(Error))]
#[repr(C)]
pub enum Error {
    /// Failed to initialize the MIDI input system: {0}
    MidirInit(#[facet(opaque, error::from)] midir::InitError),
    /// Failed to get the name of the MIDI port: {0}
    MidirPortInfo(#[facet(opaque, error::from)] midir::PortInfoError),
    /// Failed to connect to the MIDI input port: {0}
    MidirInputConnectError(#[facet(opaque, error::from)] midir::ConnectError<midir::MidiInput>),
    /// Empty MIDI message buffer.
    EmptyMidiMessageBuffer,
    /// Incomplete MIDI message.
    IncompleteMidiMessage,
    /// Invalid data byte in MIDI message: {0}
    InvalidDataByte(u8),
    /// Invalid status byte in MIDI message: {0}
    InvalidStatusByte(u8),
    /// Invalid MIDI value
    InvalidMidiValue,
}

pub type Result<T> = std::result::Result<T, Error>;
