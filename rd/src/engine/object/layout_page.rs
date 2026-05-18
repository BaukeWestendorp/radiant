use gpui::Bounds;

use crate::engine::{Object, ObjectId, ObjectKind, SlotId};

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LayoutPage {
    id: ObjectId,
    slot_id: SlotId,
    name: String,

    tiles: Vec<LayoutTile>,
}

impl LayoutPage {
    pub fn tiles(&self) -> &[LayoutTile] {
        &self.tiles
    }
}

impl Object for LayoutPage {
    fn kind() -> ObjectKind {
        ObjectKind::LayoutPage
    }

    fn id(&self) -> ObjectId {
        self.id
    }

    fn slot_id(&self) -> SlotId {
        self.slot_id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LayoutTile {
    bounds: Bounds<u32>,
    kind: LayoutTileKind,
}

impl LayoutTile {
    pub fn bounds(&self) -> Bounds<u32> {
        self.bounds
    }

    pub fn kind(&self) -> &LayoutTileKind {
        &self.kind
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum LayoutTileKind {
    Fixtures,

    GroupPool,
    EffectPool,
    CueListPool,
}
