use std::fmt;

use crate::{ExecutorButton, ObjectId, Slot};

pub mod midi;

#[derive(Default, Clone)]
#[derive(facet::Facet)]
#[facet(deny_unknown_fields)]
pub struct TriggerConfig {
    pub midi: Vec<midi::MidiMapping>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[derive(facet::Facet)]
#[facet(tag = "type")]
#[repr(C)]
pub enum TriggerTarget {
    HighlightToggle,
    ExecutorMaster {
        #[facet(rename = "page")]
        page_id: ObjectId,
        slot: Slot,
    },
    ExecutorButton {
        #[facet(rename = "page")]
        page_id: ObjectId,
        slot: Slot,
        button: ExecutorButton,
    },
    Encoder {
        #[facet(rename = "ix")]
        encoder_ix: usize,
    },
}

impl fmt::Display for TriggerTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TriggerTarget::HighlightToggle => write!(f, "Highlight Toggle"),
            TriggerTarget::ExecutorMaster { page_id, slot } => {
                write!(f, "Executor Master (Page: {}, Slot: {})", page_id, slot)
            }
            TriggerTarget::ExecutorButton { page_id, slot, button } => {
                write!(
                    f,
                    "Executor Button (Page: {}, Slot: {}, Button: {:?})",
                    page_id, slot, button
                )
            }
            TriggerTarget::Encoder { encoder_ix } => {
                write!(f, "Encoder (Index: {})", encoder_ix)
            }
        }
    }
}
