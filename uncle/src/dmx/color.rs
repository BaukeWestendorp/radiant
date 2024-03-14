use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DmxColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl DmxColor {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl From<DmxColor> for gpui::Rgba {
    fn from(value: DmxColor) -> Self {
        gpui::Rgba {
            r: value.red as f32 / 255.0,
            g: value.green as f32 / 255.0,
            b: value.blue as f32 / 255.0,
            a: 1.0,
        }
    }
}
