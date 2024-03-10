use gpui::SharedString;

use crate::workspace::layout::LayoutBounds;

use self::presets::Presets;

pub mod presets;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Show {
    pub name: SharedString,
    pub presets: Presets,
    pub layout: Layout,
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Layout {
    pub windows: Vec<Window>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Window {
    pub bounds: LayoutBounds,
    pub kind: WindowKind,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum WindowKind {
    Pool,
}

impl WindowKind {
    pub fn window_title(&self) -> &str {
        match self {
            WindowKind::Pool => "Pool Window",
        }
    }

    pub fn show_header(&self) -> bool {
        match self {
            WindowKind::Pool => false,
        }
    }
}
