use std::{fmt, num::NonZeroU32};

use uuid::Uuid;

pub use executor::*;

mod executor;

pub trait Object: for<'facet> facet::Facet<'facet> {
    fn slot(&self) -> Slot;

    fn id(&self) -> ObjectId;

    fn name(&self) -> &str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(facet::Facet)]
#[facet(transparent)]
pub struct ObjectId(Uuid);

impl ObjectId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn random() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(facet::Facet)]
#[facet(transparent)]
pub struct Slot(NonZeroU32);

impl Slot {
    pub fn new(nz: NonZeroU32) -> Self {
        Self(nz)
    }

    pub fn as_u32(&self) -> u32 {
        self.0.into()
    }
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
