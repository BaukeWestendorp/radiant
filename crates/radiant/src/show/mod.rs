use std::collections::HashMap;

use anyhow::{anyhow, Result};
use assets::Assets;
use gdtf::fixture_type::dmx_modes::{DmxChannel, DmxMode};
use gdtf::fixture_type::FixtureType;
use gdtf::GdtfDescription;
use gpui::{AssetSource, SharedString};

use crate::workspace::layout::LayoutBounds;

use self::presets::Presets;

pub mod presets;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Show {
    pub name: SharedString,
    pub presets: Presets,
    pub layout: Layout,
    pub patch_list: PatchList,
}

impl Show {
    pub fn from_file(path: &str) -> Result<Self> {
        let show_json = std::fs::read_to_string(path)
            .map_err(|_| anyhow!("Failed to read show file '{}'", path))?;
        let loaded_show = serde_json::from_str(&show_json)
            .map_err(|_| anyhow!("Failed to parse show file '{}'", path))?;
        Ok(loaded_show)
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let show_json = serde_json::to_string_pretty(self)
            .map_err(|_| anyhow!("Failed to serialize show to json"))?;
        std::fs::write(path, show_json)
            .map_err(|_| anyhow!("Failed to write show file '{}'", path))?;
        Ok(())
    }
}

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
    ColorPicker(ColorPickerWindow),
    FixtureSheet(FixtureSheetWindow),
}

impl WindowKind {
    pub fn window_title(&self) -> &str {
        match self {
            WindowKind::Pool(_) => "Pool Window",
            WindowKind::ColorPicker(_) => "Color Picker",
            WindowKind::FixtureSheet(_) => "Fixture Sheet",
        }
    }

    pub fn show_header(&self) -> bool {
        match self {
            WindowKind::Pool(_) => false,
            WindowKind::ColorPicker(_) => true,
            WindowKind::FixtureSheet(_) => true,
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

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct ColorPickerWindow {}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct FixtureSheetWindow {}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Programmer {}

#[derive(Debug, Clone, serde::Deserialize, Default, serde::Serialize)]
pub struct PatchList {
    pub fixtures: Vec<Fixture>,

    fixture_types: HashMap<FixtureTypeId, String>,

    #[serde(skip_serializing, default = "HashMap::new")]
    fixture_type_cache: HashMap<String, FixtureType>,
}

pub type FixtureTypeId = usize;

impl PatchList {
    pub fn get_fixture_type(&mut self, id: FixtureTypeId) -> Option<&FixtureType> {
        let file_name = match self.fixture_types.get(&id) {
            Some(file_name) => file_name,
            None => {
                log::error!("Fixture type not found for id '{}'.", id);
                return None;
            }
        };

        if !self.fixture_type_cache.contains_key(file_name) {
            let path = format!("fixtures/{}", file_name);
            let gdtf_description = load_gdtf_description(&path);
            let fixture_type = gdtf_description.fixture_type;
            self.fixture_type_cache
                .insert(file_name.clone(), fixture_type.clone());
        }

        self.fixture_type_cache.get(file_name)
    }

    pub fn register_fixture_type(&mut self, file_name: &str) -> FixtureTypeId {
        let id = self.new_fixture_type_id();
        self.fixture_types.insert(id, file_name.to_string());
        id
    }

    fn new_fixture_type_id(&self) -> FixtureTypeId {
        // TODO: This is not a good way to get a new id. This only works if you can't
        // remove fixture types.
        self.fixture_types.len()
    }
}

fn load_gdtf_description(path: &str) -> GdtfDescription {
    let fixture_file = Assets
        .load(&path)
        .expect(format!("Fixture asset not found at path '{}'", path).as_str());
    GdtfDescription::from_archive_bytes(&fixture_file)
        .expect(format!("Failed to parse GDTF file at path '{}'", path).as_str())
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Fixture {
    pub id: Option<FixtureId>,
    pub name: SharedString,
    pub type_id: FixtureTypeId,
    pub mode_index: u8,
    pub patch: Option<Patch>,
}

impl Fixture {
    pub fn new(
        id: Option<FixtureId>,
        name: SharedString,
        type_id: FixtureTypeId,
        mode_index: u8,
        patch: Option<Patch>,
    ) -> Self {
        Self {
            id,
            name,
            type_id,
            mode_index,
            patch,
        }
    }

    pub fn fixture_type<'a>(&'a self, patch_list: &'a mut PatchList) -> &FixtureType {
        match patch_list.get_fixture_type(self.type_id) {
            Some(fixture_type) => fixture_type,
            None => {
                log::error!("Fixture type not found for id '{}'.", self.type_id);
                panic!()
            }
        }
    }

    pub fn mode<'a>(&'a self, patch_list: &'a mut PatchList) -> &DmxMode {
        let fixture_type = self.fixture_type(patch_list);
        match fixture_type.dmx_modes.modes.get(self.mode_index as usize) {
            Some(mode) => &mode,
            None => {
                log::error!(
                    "Mode not found for index '{}' in fixture type '{}'.",
                    self.mode_index,
                    fixture_type.name
                );
                panic!()
            }
        }
    }

    pub fn get_valid_channels<'a>(
        &'a self,
        patch_list: &'a mut PatchList,
    ) -> impl Iterator<Item = &DmxChannel> {
        let mode = self.mode(patch_list);
        mode.dmx_channels
            .channels
            .iter()
            .filter(|c| c.offset.clone().is_some_and(|o| !o.is_empty()))
    }
}

pub type FixtureId = usize;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Patch {
    pub address: u16,
    pub universe: u8,
}

impl std::fmt::Display for Patch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{:03}", self.universe, self.address)
    }
}
