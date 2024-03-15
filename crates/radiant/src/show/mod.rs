use std::collections::HashMap;

use gdtf::fixture_type::dmx_modes::{DmxChannel, DmxMode};
use gdtf::fixture_type::FixtureType;
use gdtf::GdtfDescription;
use gpui::SharedString;

use crate::workspace::layout::LayoutBounds;

use self::presets::Presets;

pub mod presets;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Show {
    pub name: SharedString,
    pub presets: Presets,
    pub layout: Layout,
    pub patch: Patch,
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
pub struct Patch {
    pub fixtures: Vec<Fixture>,

    fixture_types: HashMap<FixtureTypeId, String>,

    #[serde(skip_serializing, default = "HashMap::new")]
    fixture_type_cache: HashMap<String, FixtureType>,
}

pub type FixtureTypeId = usize;

impl Patch {
    pub fn get_fixture_type(&mut self, id: FixtureTypeId) -> Option<FixtureType> {
        let file_name = self.fixture_types.get(&id).unwrap();
        if let Some(fixture_type) = self.fixture_type_cache.get(file_name) {
            return Some(fixture_type.clone());
        }

        let root = std::env::current_dir().unwrap();
        let path = root.join("assets").join("fixtures").join(file_name);
        let file = std::fs::File::open(path).unwrap();
        let description = GdtfDescription::from_archive_file(&file).unwrap();
        let fixture_type = description.fixture_type;

        self.fixture_type_cache
            .insert(file_name.clone(), fixture_type.clone());

        Some(fixture_type)
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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Fixture {
    pub id: FixtureId,
    pub name: SharedString,
    pub r#type: FixtureTypeId,
    pub universe: u16,
    pub address: u16,
    pub mode_index: u8,
}

impl Fixture {
    pub fn get_mode(&self, patch: &mut Patch) -> DmxMode {
        let fixture_type = patch.get_fixture_type(self.r#type).unwrap();
        fixture_type
            .dmx_modes
            .modes
            .get(self.mode_index as usize)
            .unwrap()
            .clone()
    }

    pub fn get_valid_channels(&self, patch: &mut Patch) -> Vec<DmxChannel> {
        let mode = self.get_mode(patch);
        mode.dmx_channels
            .channels
            .into_iter()
            .filter(|c| c.offset.clone().is_some_and(|o| !o.is_empty()))
            .collect()
    }
}

pub type FixtureId = usize;
