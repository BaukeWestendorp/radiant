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
#[cfg_attr(feature = "rd-ui", derive(rd_ui::Input))]
#[facet(tag = "type")]
#[repr(C)]
pub enum TriggerTarget {
    HighlightToggle,
    ExecutorMaster {
        #[facet(rename = "page")]
        #[rd_ui(field_name = "Page")]
        page_id: ObjectId,
        #[rd_ui(field_name = "Slot")]
        slot: Slot,
    },
    ExecutorButton {
        #[facet(rename = "page")]
        #[rd_ui(field_name = "Page")]
        page_id: ObjectId,
        #[rd_ui(field_name = "Slot")]
        slot: Slot,
        #[rd_ui(field_name = "Button")]
        button: ExecutorButton,
    },
    Encoder {
        #[facet(rename = "ix")]
        #[rd_ui(field_name = "Encoder")]
        encoder_ix: usize,
    },
}

impl fmt::Display for TriggerTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TriggerTarget::HighlightToggle => write!(f, "Highlight Toggle"),
            TriggerTarget::ExecutorMaster { page_id, slot } => {
                write!(f, "Executor Master {}.{}", page_id, slot)
            }
            TriggerTarget::ExecutorButton { page_id, slot, button } => {
                write!(f, "Executor Button {}.{}.{:?})", page_id, slot, button)
            }
            TriggerTarget::Encoder { encoder_ix } => {
                write!(f, "Encoder {}", encoder_ix)
            }
        }
    }
}
