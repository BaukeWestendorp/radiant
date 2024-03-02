use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DmxColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl DmxColor {}

impl Into<gpui::Rgba> for DmxColor {
    fn into(self) -> gpui::Rgba {
        gpui::Rgba {
            r: self.red as f32 / 255.0,
            g: self.green as f32 / 255.0,
            b: self.blue as f32 / 255.0,
            a: 1.0,
        }
    }
}
