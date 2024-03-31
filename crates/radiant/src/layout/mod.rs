use std::collections::HashMap;

pub mod screen;
pub mod window;
pub mod window_grid;

pub use screen::*;
pub use window::*;
pub use window_grid::*;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Layout {
    window_grids: Vec<WindowGrid>,
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct WindowGrid {
    windows: HashMap<usize, Window>,
}

impl WindowGrid {
    pub fn new() -> Self {
        Self::default()
    }

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
    pub bounds: GridBounds,
    pub kind: WindowKind,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum WindowKind {
    Pool(PoolWindow),
    Executors,
    FixtureSheet,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct PoolWindow {
    pub kind: PoolWindowKind,
    pub scroll_offset: i32,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum PoolWindowKind {
    ColorPreset,
    Group,
}
