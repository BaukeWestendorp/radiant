use crate::object::{ExecutorButton, ExecutorId};

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TriggersDefinition {
    midi: Vec<MidiTriggerDefinition>,
}

impl TriggersDefinition {
    pub fn midi(&self) -> &[MidiTriggerDefinition] {
        &self.midi
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MidiTriggerDefinition {
    device_name: String,
    channel: MultiRange<u4>,
    message: MidiMessage,
    target: TriggerTarget,
}

impl MidiTriggerDefinition {
    pub fn device_name(&self) -> &str {
        &self.device_name
    }

    pub fn channel(&self) -> &MultiRange<u4> {
        &self.channel
    }

    pub fn trigger(&self) -> &MidiMessage {
        &self.message
    }

    pub fn target(&self) -> &TriggerTarget {
        &self.target
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum MidiMessage {
    NoteOff { note: MultiRange<u7>, velocity: MultiRange<u7> },
    NoteOn { note: MultiRange<u7>, velocity: MultiRange<u7> },
    PolyphonicAftertouch { note: MultiRange<u7>, pressure: MultiRange<u7> },
    ControlChange { controller: MultiRange<u7>, value: MultiRange<u7> },
    ProgramChange { program: MultiRange<u7> },
    ChannelAftertouch { pressure: MultiRange<u7> },
    PitchBend { value: MultiRange<i16> },
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum TriggerTarget {
    ExecutorMaster { executor_id: ExecutorId },
    ExecutorButton { executor_id: ExecutorId, button: ExecutorButton },
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MultiRange<T> {
    Single(T),
    Range { from: T, to: T },
    Selection(Vec<MultiRange<T>>),
}

impl<T: PartialOrd + Copy> MultiRange<T> {
    pub fn contains(&self, value: &T) -> bool {
        match self {
            MultiRange::Single(v) => v == value,
            MultiRange::Range { from, to } => from <= value && value <= to,
            MultiRange::Selection(ranges) => ranges.iter().any(|r| r.contains(value)),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct u7(u8);

impl u7 {
    pub fn new(value: u8) -> anyhow::Result<Self> {
        if value <= 127 {
            Ok(u7(value))
        } else {
            Err(anyhow::anyhow!("value out of range for u7: {}", value))
        }
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl From<midly::num::u7> for u7 {
    fn from(value: midly::num::u7) -> Self {
        u7(value.as_int())
    }
}

impl<'de> serde::Deserialize<'de> for u7 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        u7::new(value).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for u7 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.0)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct u4(u8);

impl u4 {
    pub fn new(value: u8) -> anyhow::Result<Self> {
        if value <= 15 {
            Ok(u4(value))
        } else {
            Err(anyhow::anyhow!("value out of range for u4: {}", value))
        }
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl From<midly::num::u4> for u4 {
    fn from(value: midly::num::u4) -> Self {
        u4(value.as_int())
    }
}

impl<'de> serde::Deserialize<'de> for u4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        u4::new(value).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for u4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.0)
    }
}
