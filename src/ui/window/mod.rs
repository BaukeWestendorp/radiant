use serde::{Deserialize, Serialize};

pub mod color_preset_window;

pub use color_preset_window::ColorPresetWindow;

#[derive(Clone, Serialize, Deserialize)]
pub struct Window {
    pub kind: WindowKind,
}

impl Window {
    pub fn new(kind: WindowKind) -> Self {
        Self { kind }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum WindowKind {
    ColorPreset(ColorPresetWindow),
}
