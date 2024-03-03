use std::collections::HashMap;

use gpui::{Global, SharedString};
use serde::{Deserialize, Serialize};

use crate::{dmx::color::DmxColor, ui::screen::Screen};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Show {
    screen: Screen,
    presets: Presets,

    #[serde(skip_serializing)]
    programmer_state: ProgrammerState,
}

impl Show {
    pub fn new() -> Self {
        Self {
            screen: Screen::new(),
            presets: Presets::new(),
            programmer_state: ProgrammerState::default(),
        }
    }

    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    pub fn screen_mut(&mut self) -> &mut Screen {
        &mut self.screen
    }

    pub fn presets(&self) -> &Presets {
        &self.presets
    }

    pub fn presets_mut(&mut self) -> &mut Presets {
        &mut self.presets
    }

    pub fn programmer_state(&self) -> ProgrammerState {
        self.programmer_state
    }

    pub fn set_programmer_state(&mut self, programmer_state: ProgrammerState) {
        self.programmer_state = programmer_state;
    }
}

impl Global for Show {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Presets {
    colors: HashMap<ColorPresetId, ColorPreset>,
}

impl Presets {
    pub fn new() -> Self {
        Self {
            colors: HashMap::new(),
        }
    }

    pub fn add_color_preset(&mut self, color_preset: ColorPreset) -> ColorPresetId {
        let id = self.get_new_color_id();
        self.colors.insert(id, color_preset);
        id
    }

    pub fn color_preset(&self, id: ColorPresetId) -> &ColorPreset {
        self.colors.get(&id).unwrap()
    }

    pub fn color_preset_mut(&mut self, id: ColorPresetId) -> &mut ColorPreset {
        self.colors.get_mut(&id).unwrap()
    }

    pub fn color_presets(&self) -> impl Iterator<Item = (ColorPresetId, &ColorPreset)> {
        self.colors.iter().map(|(id, preset)| (*id, preset))
    }

    fn get_new_color_id(&self) -> ColorPresetId {
        // TODO: This is not a good way to get a new id. This only works if you can't remove colors.
        ColorPresetId(self.colors.len() as usize)
    }
}

pub trait Preset {
    fn label(&self) -> &str;

    fn set_label(&mut self, label: &str);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ColorPresetId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ProgrammerState {
    #[default]
    Normal,
    Store,
}

impl ToString for ProgrammerState {
    fn to_string(&self) -> String {
        match self {
            ProgrammerState::Normal => "Normal".to_string(),
            ProgrammerState::Store => "Store".to_string(),
        }
    }
}
