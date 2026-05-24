use crate::{ObjectReference, SlotId};

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    trigger_mapping: TriggerMapping,
}

impl Config {
    pub fn trigger_mapping(&self) -> &TriggerMapping {
        &self.trigger_mapping
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TriggerMapping {
    midi: Vec<MidiDeviceMapping>,
}

impl TriggerMapping {
    pub fn midi(&self) -> &[MidiDeviceMapping] {
        &self.midi
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MidiDeviceMapping {
    midi_device: String,
    triggers: Vec<MidiTrigger>,
}

impl MidiDeviceMapping {
    pub fn midi_device(&self) -> &str {
        &self.midi_device
    }

    pub fn triggers(&self) -> &[MidiTrigger] {
        &self.triggers
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MidiTrigger {
    target: MidiTriggerTarget,
    event: MidiTriggerEvent,
}

impl MidiTrigger {
    pub fn target(&self) -> &MidiTriggerTarget {
        &self.target
    }

    pub fn event(&self) -> &MidiTriggerEvent {
        &self.event
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MidiTriggerTarget {
    Executor { page: ObjectReference, executor: SlotId },
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MidiTriggerEvent {
    ControlChange(u8),
}
