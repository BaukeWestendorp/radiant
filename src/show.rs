use std::collections::HashMap;

use gpui::Global;
use palette::Srgb;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Show {
    colors: HashMap<ObjectId, Srgb>,
    groups: HashMap<ObjectId, LedGroup>,
}

impl Show {
    pub fn new() -> Self {
        Self {
            colors: HashMap::new(),
            groups: HashMap::new(),
        }
    }

    pub fn add_color(&mut self, color: Srgb) -> ObjectId {
        let id = ObjectId::new();
        self.colors.insert(id, color);
        id
    }

    pub fn add_group(&mut self, group: LedGroup) -> ObjectId {
        let id = ObjectId::new();
        self.groups.insert(id, group);
        id
    }

    pub fn get_color(&self, id: &ObjectId) -> Option<&Srgb> {
        self.colors.get(id)
    }

    pub fn get_group(&self, id: &ObjectId) -> Option<&LedGroup> {
        self.groups.get(id)
    }
}

impl Global for Show {}

#[derive(Debug, Clone)]
pub struct LedGroup {
    led_ids: Vec<LedIndex>,
}

impl LedGroup {
    pub fn new(led_ids: Vec<LedIndex>) -> Self {
        Self { led_ids }
    }

    pub fn led_ids(&self) -> &[LedIndex] {
        &self.led_ids
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId(Uuid);

impl ObjectId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn uuid(&self) -> &Uuid {
        &self.0
    }
}

pub type LedIndex = usize;
