use std::fmt;

use crate::project::TriggerTarget;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[derive(facet::Facet)]
#[facet(deny_unknown_fields)]
pub struct MidiMapping {
    pub device_name: String,

    /// `None` will allow any channel.
    pub device_channel: Option<u8>,

    /// What incoming MIDI events should trigger this?
    pub filter_type: FilterType,

    /// `None` will allow any controller.
    pub filter_controller: Option<u8>,

    pub filter_note: Option<u8>,

    pub transform_min_output: f32,
    pub transform_max_output: f32,
    /// How should we scale/invert the value before sending it?
    pub transform_invert: bool,

    /// What internal action to take.
    pub target: TriggerTarget,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[derive(facet::Facet)]
#[repr(C)]
pub enum FilterType {
    ControlChange,
    NoteOn,
    NoteOff,
    PitchBend,
}

impl fmt::Display for FilterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilterType::ControlChange => write!(f, "Control Change"),
            FilterType::NoteOn => write!(f, "Note On"),
            FilterType::NoteOff => write!(f, "Note Off"),
            FilterType::PitchBend => write!(f, "Pitch Bend"),
        }
    }
}
