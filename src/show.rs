use std::collections::HashMap;

use gpui::Global;
use serde::{Deserialize, Serialize};

use crate::{
    dmx::color::DmxColor,
    ui::layout::{Layout, LayoutId},
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Show {
    layouts: HashMap<LayoutId, Layout>,
    presets: Presets,
}

impl Show {
    pub fn new() -> Self {
        Self {
            layouts: HashMap::new(),
            presets: Presets::new(),
        }
    }

    pub fn add_layout(&mut self, layout: Layout) -> LayoutId {
        let id = self.get_new_layout_id();
        self.layouts.insert(id, layout);
        id
    }

    fn get_new_layout_id(&self) -> LayoutId {
        // TODO: This is not a good way to get a new id. This only works if you can't remove layouts.
        LayoutId(self.layouts.len() as usize)
    }

    pub fn layout(&self, id: LayoutId) -> &Layout {
        self.layouts.get(&id).unwrap()
    }

    pub fn layout_mut(&mut self, id: LayoutId) -> &mut Layout {
        self.layouts.get_mut(&id).unwrap()
    }
}

impl Global for Show {}

#[derive(Clone, Serialize, Deserialize)]
pub struct Presets {
    pub colors: HashMap<ColorPresetId, ColorPreset>,
}

impl Presets {
    pub fn new() -> Self {
        Self {
            colors: HashMap::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ColorPreset {
    pub color: DmxColor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ColorPresetId(pub(crate) usize);
