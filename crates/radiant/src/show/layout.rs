use std::collections::HashMap;

use crate::workspace::layout::LayoutBounds;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Layout {
    windows: HashMap<usize, Window>,
}

impl Layout {
    pub fn window(&self, id: usize) -> Option<&Window> {
        self.windows.get(&id)
    }

    pub fn window_mut(&mut self, id: usize) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    pub fn pool_window(&self, id: usize) -> Option<&PoolWindow> {
        self.window(id).and_then(|window| match &window.kind {
            WindowKind::Pool(pool_window) => Some(pool_window),
            _ => None,
        })
    }

    pub fn pool_window_mut(&mut self, id: usize) -> Option<&mut PoolWindow> {
        self.window_mut(id)
            .and_then(|window| match &mut window.kind {
                WindowKind::Pool(pool_window) => Some(pool_window),
                _ => None,
            })
    }

    pub fn add_window(&mut self, window: Window) -> usize {
        let id = self.new_window_id();
        self.windows.insert(id, window);
        id
    }

    pub fn window_ids(&self) -> Vec<usize> {
        self.windows.keys().cloned().collect()
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

impl WindowKind {
    pub fn window_title(&self) -> &str {
        match self {
            WindowKind::Pool(_) => "Pool Window",
            WindowKind::ColorPicker => "Color Picker",
            WindowKind::FixtureSheet => "Fixture Sheet",
        }
    }

    pub fn show_header(&self) -> bool {
        match self {
            WindowKind::Pool(_) => false,
            WindowKind::ColorPicker => true,
            WindowKind::FixtureSheet => true,
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct PoolWindow {
    pub kind: PoolWindowKind,
    pub scroll_offset: i32,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum PoolWindowKind {
    Color,
}

impl PoolWindowKind {
    pub fn window_title(&self) -> &str {
        match &self {
            PoolWindowKind::Color => "Color",
        }
    }

    pub fn color(&self) -> gpui::Rgba {
        match &self {
            PoolWindowKind::Color => gpui::rgb(0x27D0CD),
        }
    }
}
