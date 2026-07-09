use crate::{ExecutorButton, ObjectId, Slot};

pub mod midi;

#[derive(Default, Clone)]
#[derive(facet::Facet)]
pub struct TriggerConfig {
    pub midi: midi::MidiTriggerConfig,
}

#[derive(Debug, Clone)]
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
