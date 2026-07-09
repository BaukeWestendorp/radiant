use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub struct MidiPacket {
    pub port_name: String,
    pub port_id: String,
    pub timestamp: Instant,
    pub message: MidiMessage,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MidiMessage {
    NoteOff { channel: u8, note: u8, velocity: u8 },
    NoteOn { channel: u8, note: u8, velocity: u8 },
    PolyphonicKeyPressure { channel: u8, note: u8, pressure: u8 },
    ControlChange { channel: u8, controller: u8, value: u8 },
    ProgramChange { channel: u8, program: u8 },
    ChannelPressure { channel: u8, pressure: u8 },
    PitchBend { channel: u8, value: i16 },

    SysEx(Vec<u8>),
    TimeCodeQuarterFrame { value: u8 },
    SongPositionPointer { beats: u16 },
    SongSelect { song: u8 },
    TuneRequest,

    TimingClock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    SystemReset,
}

impl MidiMessage {
    pub fn decode(bytes: &[u8]) -> crate::Result<Self> {
        if bytes.is_empty() {
            return Err(crate::Error::EmptyMidiMessageBuffer);
        }

        let status = bytes[0];

        macro_rules! validate_data {
            ($idx:expr) => {
                if $idx >= bytes.len() {
                    return Err(crate::Error::IncompleteMidiMessage);
                } else if bytes[$idx] >= 0x80 {
                    return Err(crate::Error::InvalidDataByte(bytes[$idx]));
                }
            };
        }

        if status >= 0xF0 {
            return match status {
                0xF0 => Ok(MidiMessage::SysEx(bytes.to_vec())),
                0xF1 => {
                    validate_data!(1);
                    Ok(MidiMessage::TimeCodeQuarterFrame { value: bytes[1] })
                }
                0xF2 => {
                    validate_data!(1);
                    validate_data!(2);
                    let beats = ((bytes[2] as u16) << 7) | (bytes[1] as u16);
                    Ok(MidiMessage::SongPositionPointer { beats })
                }
                0xF3 => {
                    validate_data!(1);
                    Ok(MidiMessage::SongSelect { song: bytes[1] })
                }
                0xF6 => Ok(MidiMessage::TuneRequest),
                0xF8 => Ok(MidiMessage::TimingClock),
                0xFA => Ok(MidiMessage::Start),
                0xFB => Ok(MidiMessage::Continue),
                0xFC => Ok(MidiMessage::Stop),
                0xFE => Ok(MidiMessage::ActiveSensing),
                0xFF => Ok(MidiMessage::SystemReset),
                _ => Err(crate::Error::InvalidStatusByte(status)),
            };
        }

        let message_type = status & 0xF0;
        let channel = status & 0x0F;

        match message_type {
            0x80 => {
                validate_data!(1);
                validate_data!(2);
                Ok(MidiMessage::NoteOff { channel, note: bytes[1], velocity: bytes[2] })
            }
            0x90 => {
                validate_data!(1);
                validate_data!(2);
                Ok(MidiMessage::NoteOn { channel, note: bytes[1], velocity: bytes[2] })
            }
            0xA0 => {
                validate_data!(1);
                validate_data!(2);
                Ok(MidiMessage::PolyphonicKeyPressure {
                    channel,
                    note: bytes[1],
                    pressure: bytes[2],
                })
            }
            0xB0 => {
                validate_data!(1);
                validate_data!(2);
                Ok(MidiMessage::ControlChange { channel, controller: bytes[1], value: bytes[2] })
            }
            0xC0 => {
                validate_data!(1);
                Ok(MidiMessage::ProgramChange { channel, program: bytes[1] })
            }
            0xD0 => {
                validate_data!(1);
                Ok(MidiMessage::ChannelPressure { channel, pressure: bytes[1] })
            }
            0xE0 => {
                validate_data!(1);
                validate_data!(2);
                let lsb = bytes[1] as i16;
                let msb = bytes[2] as i16;
                let value = (msb << 7) | lsb;
                Ok(MidiMessage::PitchBend { channel, value: value - 8192 })
            }
            _ => Err(crate::Error::InvalidStatusByte(status)),
        }
    }
}
