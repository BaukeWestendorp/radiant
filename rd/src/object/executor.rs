use std::fmt;

use crate::{ObjectId, Slot};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(facet::Facet)]
#[repr(C)]
pub enum ExecutorButton {
    #[default]
    Button1,
    Button2,
    Button3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(facet::Facet)]
#[facet(tag = "type")]
#[repr(C)]
pub enum ExecutorButtonAction {
    ToggleEnabled,
    SetEnabled { value: bool },
    FlashMaster,
    CueGoNext,
    CueGoPrevious,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(facet::Facet)]
pub struct ExecutorId {
    pub page: ObjectId,
    pub slot: Slot,
}

impl ExecutorId {
    pub fn new(page: ObjectId, slot: Slot) -> Self {
        Self { page, slot }
    }
}

impl fmt::Display for ExecutorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.page, self.slot)
    }
}
