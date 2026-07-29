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
        #[cfg_attr(feature = "rd-ui", rd_ui(label = "Page"))]
        page_id: ObjectId,
        #[cfg_attr(feature = "rd-ui", rd_ui(label = "Slot"))]
        slot: Slot,
    },
    ExecutorButton {
        #[facet(rename = "page")]
        #[cfg_attr(feature = "rd-ui", rd_ui(label = "Page"))]
        page_id: ObjectId,
        #[cfg_attr(feature = "rd-ui", rd_ui(label = "Slot"))]
        slot: Slot,
        #[cfg_attr(feature = "rd-ui", rd_ui(label = "Button"))]
        button: ExecutorButton,
    },
    Encoder {
        #[facet(rename = "ix")]
        #[cfg_attr(feature = "rd-ui", rd_ui(label = "Encoder"))]
        encoder_ix: usize,
    },
}

impl Default for TriggerTarget {
    fn default() -> Self {
        TriggerTarget::HighlightToggle
    }
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
