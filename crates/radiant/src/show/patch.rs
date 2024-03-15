use std::collections::HashMap;

use assets::{AssetSource, Assets};
use gdtf::fixture_type::dmx_modes::{DmxChannel, DmxMode};
use gdtf::fixture_type::FixtureType;
use gdtf::GdtfDescription;
use gpui::SharedString;

pub type FixtureTypeId = usize;
pub type FixtureId = usize;

#[derive(Debug, Clone, serde::Deserialize, Default, serde::Serialize)]
pub struct PatchList {
    pub fixtures: Vec<Fixture>,

    fixture_types: HashMap<FixtureTypeId, String>,

    #[serde(skip_serializing, default = "HashMap::new")]
    fixture_type_cache: HashMap<String, FixtureType>,
}

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
