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
    /// `None` will allow any channel.
    pub channel: Option<u8>,
    pub mappings: Vec<MidiMapping>,
}

#[derive(Debug, Clone)]
#[derive(facet::Facet)]
pub struct MidiMapping {
    /// What incoming MIDI events should trigger this?
    pub filter: MidiFilter,

    /// How should we scale/invert the value before sending it?
    #[facet(default)]
    pub transform: ValueTransform,

    /// What internal action to take
    pub target: TriggerTarget,
}

#[derive(Debug, Clone)]
#[derive(facet::Facet)]
#[facet(tag = "type")]
#[repr(C)]
pub enum MidiFilter {
    ControlChange {
        /// `None` will allow any controller.
        controller: Option<u8>,
    },
    NoteOn {
        note: Option<u8>,
    },
    NoteOff {
        note: Option<u8>,
    },
    PitchBend,
}

#[derive(Debug, Clone)]
#[derive(facet::Facet)]
pub struct ValueTransform {
    #[facet(default = false)]
    pub invert: bool,
    #[facet(default = 1.0)]
    pub max_output: f32,
    #[facet(default = 0.0)]
    pub min_output: f32,
}

impl Default for ValueTransform {
    fn default() -> Self {
        Self { invert: false, max_output: 1.0, min_output: 0.0 }
    }
}
