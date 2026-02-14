use crate::object::{ObjectId, Slot};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("slot {0:?} is already occupied")]
    SlotOccupied(Slot),

    #[error("slot {0:?} is empty")]
    SlotEmpty(Slot),

    #[error("object {0:?} not found")]
    ObjectNotFound(ObjectId),

    #[error("slot is zero")]
    ZeroSlot,
}
