use gpui::SharedString;

use self::presets::Presets;

pub mod presets;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Show {
    pub name: SharedString,
    pub presets: Presets,
}
