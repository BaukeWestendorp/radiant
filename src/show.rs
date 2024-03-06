use gpui::SharedString;

use crate::presets::Presets;

pub mod cmd {
    use gpui::actions;

    actions!(show_cmd, [Store]);
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Show {
    pub presets: Presets,
    pub programmer_state: SharedString,
    pub screens: Vec<Screen>,
}

impl Show {
    pub fn new() -> Self {
        Self {
            presets: Presets::new(),
            programmer_state: "normal".into(),
            screens: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Screen {
    pub layout: Layout,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Layout {
    pub windows: Vec<Window>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Window {}
