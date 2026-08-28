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
    NoteOff { channel: u4, note: u7, velocity: u7 },
    NoteOn { channel: u4, note: u7, velocity: u7 },
    PolyphonicKeyPressure { channel: u4, note: u7, pressure: u7 },
    ControlChange { channel: u4, controller: u7, value: u7 },
    ProgramChange { channel: u4, program: u7 },
    ChannelPressure { channel: u4, pressure: u7 },
    PitchBend { channel: u4, value: i16 },

    SysEx(Vec<u8>),
    TimeCodeQuarterFrame { value: u7 },
    SongPositionPointer { beats: u16 },
    SongSelect { song: u7 },
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
                    Ok(MidiMessage::TimeCodeQuarterFrame { value: u7::new(bytes[1]).unwrap() })
                }
                0xF2 => {
                    validate_data!(1);
                    validate_data!(2);
                    let beats = ((bytes[2] as u16) << 7) | (bytes[1] as u16);
                    Ok(MidiMessage::SongPositionPointer { beats })
                }
                0xF3 => {
                    validate_data!(1);
                    Ok(MidiMessage::SongSelect { song: u7::new(bytes[1]).unwrap() })
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
        let channel = u4::new(status & 0x0F).unwrap();

        match message_type {
            0x80 => {
                validate_data!(1);
                validate_data!(2);
                Ok(MidiMessage::NoteOff {
                    channel,
                    note: u7::new(bytes[1]).unwrap(),
                    velocity: u7::new(bytes[2]).unwrap(),
                })
            }
            0x90 => {
                validate_data!(1);
                validate_data!(2);
                Ok(MidiMessage::NoteOn {
                    channel,
                    note: u7::new(bytes[1]).unwrap(),
                    velocity: u7::new(bytes[2]).unwrap(),
                })
            }
            0xA0 => {
                validate_data!(1);
                validate_data!(2);
                Ok(MidiMessage::PolyphonicKeyPressure {
                    channel,
                    note: u7::new(bytes[1]).unwrap(),
                    pressure: u7::new(bytes[2]).unwrap(),
                })
            }
            0xB0 => {
                validate_data!(1);
                validate_data!(2);
                Ok(MidiMessage::ControlChange {
                    channel,
                    controller: u7::new(bytes[1]).unwrap(),
                    value: u7::new(bytes[2]).unwrap(),
                })
            }
            0xC0 => {
                validate_data!(1);
                Ok(MidiMessage::ProgramChange { channel, program: u7::new(bytes[1]).unwrap() })
            }
            0xD0 => {
                validate_data!(1);
                Ok(MidiMessage::ChannelPressure { channel, pressure: u7::new(bytes[1]).unwrap() })
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[allow(non_camel_case_types)]
pub struct u7(u8);

impl u7 {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 127;

    pub fn new(value: u8) -> Option<Self> {
        (value <= Self::MAX).then_some(Self(value))
    }

    pub fn get(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for u7 {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or("MIDI value must be between 0 and 127")
    }
}

impl std::str::FromStr for u7 {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<u8>().map_err(|_| crate::Error::InvalidMidiValue)?;
        Self::new(value).map_or(Err(crate::Error::InvalidMidiValue), Ok)
    }
}

impl std::fmt::Display for u7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[allow(non_camel_case_types)]
pub struct u4(u8);

impl u4 {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 15;

    pub fn new(value: u8) -> Option<Self> {
        (value <= Self::MAX).then_some(Self(value))
    }

    pub fn get(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for u4 {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(crate::Error::InvalidMidiValue)
    }
}

impl std::str::FromStr for u4 {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<u8>().map_err(|_| crate::Error::InvalidMidiValue)?;
        Self::new(value).map_or(Err(crate::Error::InvalidMidiValue), Ok)
    }
}

impl std::fmt::Display for u4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
