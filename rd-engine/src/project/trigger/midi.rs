use crate::project::TriggerTarget;

#[derive(Default, Clone)]
#[derive(facet::Facet)]
pub struct MidiTriggerConfig {
    pub devices: Vec<MidiDeviceConfig>,
}

#[derive(Debug, Clone)]
#[derive(facet::Facet)]
pub struct MidiDeviceConfig {
    pub name: String,
    pub channel: u8,
    pub mappings: Vec<MidiMapping>,
}

#[derive(Debug, Clone)]
#[derive(facet::Facet)]
pub struct MidiMapping {
    pub msg: MidiMessage,
    pub target: TriggerTarget,
}

#[derive(Debug, Clone)]
#[derive(facet::Facet)]
#[facet(tag = "type")]
#[repr(C)]
pub enum MidiMessage {
    ControlChange {
        controller: u8,
        #[facet(default = MidiRange { from: 0, to: 127 })]
        value: MidiRange,
    },
    NoteOn {
        note: u8,
    },
}

#[derive(Debug, Clone)]
#[derive(facet::Facet)]
pub struct MidiRange {
    pub from: u8,
    pub to: u8,
}
