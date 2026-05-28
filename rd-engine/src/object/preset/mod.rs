mod dimmer;

use std::collections::HashMap;

pub use dimmer::*;
use zeevonk::{AttributeName, value::AttributeValue};

use crate::ObjectId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PresetId {
    kind: PresetKind,
    object_id: ObjectId,
}

impl PresetId {
    pub fn new(kind: PresetKind, object_id: ObjectId) -> Self {
        Self { kind, object_id }
    }

    pub fn kind(&self) -> PresetKind {
        self.kind
    }

    pub fn object_id(&self) -> ObjectId {
        self.object_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PresetKind {
    Dimmer,
}

pub trait Preset {
    fn values(&self) -> &HashMap<AttributeName, AttributeValue>;
}
