use std::collections::HashMap;

use gpui::SharedString;

use crate::dmx::color::DmxColor;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Presets {
    colors: HashMap<usize, ColorPreset>,
}

impl Presets {
    pub fn new() -> Self {
        Self {
            colors: HashMap::new(),
        }
    }

    pub fn add_color_preset(&mut self, color_preset: ColorPreset) -> usize {
        let id = self.get_new_color_id();
        self.colors.insert(id, color_preset);
        id
    }

    pub fn set_color_preset(&mut self, id: usize, color_preset: ColorPreset) {
        self.colors.insert(id, color_preset);
    }

    pub fn color_preset(&self, id: usize) -> Option<&ColorPreset> {
        self.colors.get(&id)
    }

    pub fn color_preset_mut(&mut self, id: usize) -> Option<&mut ColorPreset> {
        self.colors.get_mut(&id)
    }

    pub fn color_presets(&self) -> impl Iterator<Item = (usize, &ColorPreset)> {
        self.colors.iter().map(|(id, preset)| (*id, preset))
    }

    fn get_new_color_id(&self) -> usize {
        // TODO: This is not a good way to get a new id. This only works if you can't
        // remove colors.
        self.colors.len()
    }
}

pub trait Preset {
    fn label(&self) -> &str;

    fn set_label(&mut self, label: &str);

    fn affected_attributes(&self) -> AffectedAttributes;
}

pub enum AffectedAttributes {
    All,
    Specific(Vec<&'static str>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ColorPreset {
    label: SharedString,
    pub color: DmxColor,
}

impl ColorPreset {
    pub fn new(label: &str, color: DmxColor) -> Self {
        Self {
            label: label.to_string().into(),
            color,
        }
    }
}

impl Preset for ColorPreset {
    fn label(&self) -> &str {
        &self.label
    }

    fn set_label(&mut self, label: &str) {
        self.label = label.to_string().into();
    }

    fn affected_attributes(&self) -> AffectedAttributes {
        AffectedAttributes::Specific(vec!["ColorAdd_R", "ColorAdd_G", "ColorAdd_B"])
    }
}
