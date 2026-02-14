use crate::object::{ObjectId, SlotId};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("slot id {0:?} is already occupied")]
    SlotOccupied(SlotId),
    #[error("slot {0:?} is empty")]
    SlotEmpty(SlotId),
    #[error("slot id is zero")]
    ZeroSlotId,

    #[error("object {0:?} not found")]
    ObjectNotFound(ObjectId),
}
