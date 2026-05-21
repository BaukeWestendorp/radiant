use crate::{Object, ObjectId, ObjectKind, SlotId};

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
    kind: LayoutTileKind,
    width: u32,
    height: u32,
    x: u32,
    y: u32,
}

impl LayoutTile {
    pub fn kind(&self) -> &LayoutTileKind {
        &self.kind
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum LayoutTileKind {
    Fixtures,
    Executors,

    GroupPool,
    EffectPool,
    CueListPool,
}
