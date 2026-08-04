use rd_midi::{u4, u7};

use crate::project::TriggerTarget;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[derive(facet::Facet)]
#[facet(deny_unknown_fields)]
pub struct MidiMapping {
    pub device_name: String,
    pub device_channel: MidiChannel,
    pub filter: MidiFilter,
    pub target: TriggerTarget,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[derive(facet::Facet)]
#[cfg_attr(feature = "rd-ui", derive(rd_ui::Input))]
#[repr(C)]
pub enum MidiFilter {
    #[cfg_attr(feature = "rd-ui", rd_ui(label = "Control Change"))]
    ControlChange {
        #[cfg_attr(feature = "rd-ui", rd_ui(label = "Controller"))]
        controller: MidiController,
    },
    #[cfg_attr(feature = "rd-ui", rd_ui(label = "Note On"))]
    NoteOn {
        #[cfg_attr(feature = "rd-ui", rd_ui(label = "Note"))]
        note: MidiNote,
    },
    #[cfg_attr(feature = "rd-ui", rd_ui(label = "Note Off"))]
    NoteOff {
        #[cfg_attr(feature = "rd-ui", rd_ui(label = "Note"))]
        note: MidiNote,
    },
    #[cfg_attr(feature = "rd-ui", rd_ui(label = "Pitch Bend"))]
    PitchBend,
}

impl Default for MidiFilter {
    fn default() -> Self {
        MidiFilter::ControlChange { controller: MidiController::All }
    }
}

impl std::fmt::Display for MidiFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MidiFilter::ControlChange { controller: MidiController::Single(c) } => {
                write!(f, "CC {}", c.get())
            }
            MidiFilter::ControlChange { controller: MidiController::All } => {
                write!(f, "CC (Any)")
            }
            MidiFilter::NoteOn { note: MidiNote::Single(n) } => {
                write!(f, "Note On ({})", n.get())
            }
            MidiFilter::NoteOn { note: MidiNote::All } => write!(f, "Note On (Any)"),
            MidiFilter::NoteOff { note: MidiNote::Single(n) } => {
                write!(f, "Note Off ({})", n.get())
            }
            MidiFilter::NoteOff { note: MidiNote::All } => write!(f, "Note Off (Any)"),
            MidiFilter::PitchBend => write!(f, "Pitch Bend"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(facet::Facet)]
#[cfg_attr(feature = "rd-ui", derive(rd_ui::Input))]
#[repr(C)]
pub enum MidiChannel {
    #[default]
    All,
    Single(#[cfg_attr(feature = "rd-ui", rd_ui(label = "Channel"))] u4),
}

impl std::fmt::Display for MidiChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MidiChannel::All => write!(f, "All"),
            MidiChannel::Single(channel) => write!(f, "{}", channel.get()),
        }
    }
}

#[cfg(feature = "rd-ui")]
impl rd_ui::EnumerableValue for MidiChannel {
    fn enumerated_value(&self, offset: usize) -> Self {
        match self {
            MidiChannel::All => MidiChannel::All,
            MidiChannel::Single(channel) => {
                let new_channel = channel.get().saturating_add(offset as u8);
                let clamped_channel = new_channel.min(u4::MAX);
                MidiChannel::Single(u4::new(clamped_channel).unwrap())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(facet::Facet)]
#[cfg_attr(feature = "rd-ui", derive(rd_ui::Input))]
#[repr(C)]
pub enum MidiNote {
    #[default]
    All,
    Single(#[cfg_attr(feature = "rd-ui", rd_ui(label = "Note"))] u7),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(facet::Facet)]
#[cfg_attr(feature = "rd-ui", derive(rd_ui::Input))]
#[repr(C)]
pub enum MidiController {
    #[default]
    All,
    Single(#[cfg_attr(feature = "rd-ui", rd_ui(label = "Controller"))] u7),
}
