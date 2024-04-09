use anyhow::{anyhow, Result};
use backstage::AttributeValues;
use gpui::Rgba;

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    pub fn from_attribute_values(values: &AttributeValues) -> Result<Self> {
        if let Some(red) = values.get("ColorAdd_R") {
            if let Some(green) = values.get("ColorAdd_G") {
                if let Some(blue) = values.get("ColorAdd_B") {
                    return Ok(Self::new(
                        red.to_fraction(),
                        green.to_fraction(),
                        blue.to_fraction(),
                    ));
                }
            }
        }

        Err(anyhow!("Failed to get color from attribute values"))
    }

    pub fn hex(&self) -> u32 {
        let r = (self.red * 255.0) as u32;
        let g = (self.green * 255.0) as u32;
        let b = (self.blue * 255.0) as u32;
        (r << 16) | (g << 8) | b
    }

    pub fn darkened(self, factor: f32) -> Self {
        Self {
            red: self.red * (1.0 - factor),
            green: self.green * (1.0 - factor),
            blue: self.blue * (1.0 - factor),
        }
    }
}

impl From<Color> for Rgba {
    fn from(value: Color) -> Self {
        Rgba {
            r: value.red,
            g: value.green,
            b: value.blue,
            a: 1.0,
        }
    }
}
