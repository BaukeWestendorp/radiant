use std::collections::HashMap;

use crate::workspace::layout::LayoutBounds;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Layout {
    windows: HashMap<usize, Window>,
}

impl Layout {
    pub fn windows(&self) -> &HashMap<usize, Window> {
        &self.windows
    }

    pub fn window(&self, id: usize) -> Option<&Window> {
        self.windows.get(&id)
    }

    pub fn window_mut(&mut self, id: usize) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    pub fn add_window(&mut self, window: Window) -> usize {
        let id = self.new_window_id();
        self.windows.insert(id, window);
        id
    }

    fn new_window_id(&self) -> usize {
        // TODO: This is not a good way to get a new id. This only works if you can't
        // remove colors.
        self.windows.len()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Window {
    pub bounds: LayoutBounds,
    pub kind: WindowKind,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum WindowKind {
    Pool(PoolWindow),
    ColorPicker,
    FixtureSheet,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct PoolWindow {
    pub kind: PoolWindowKind,
    pub scroll_offset: usize,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum PoolWindowKind {
    Color,
    Group,
}
