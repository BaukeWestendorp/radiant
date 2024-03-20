use std::collections::HashMap;

use anyhow::anyhow;
use assets::{AssetSource, Assets};
use gdtf::{Attribute, FixtureType, GdtfDescription};
use gpui::SharedString;

use crate::dmx::DmxChannel;

pub type FixtureTypeId = usize;
pub type FixtureId = usize;

#[derive(Debug, Clone, Default)]
pub struct PatchList {
    pub fixtures: Vec<Fixture>,

    // FIXME: We should use the fixture id instead of the file name.
    gdtf_descriptions: HashMap<SharedString, GdtfDescription>,
}

impl<'de> serde::Deserialize<'de> for PatchList {
    fn deserialize<D>(deserializer: D) -> Result<PatchList, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, serde::Deserialize)]
        struct Intermediate {
            fixtures: Vec<Fixture>,
        }

        let intermediate: Intermediate = serde::Deserialize::deserialize(deserializer)?;

        let fixtures = intermediate.fixtures;
        let mut gdtf_descriptions = HashMap::new();
        for fixture in fixtures.iter() {
            match load_gdtf_description(format!("fixtures/{}", fixture.gdtf_file_name).as_str()) {
                Ok(gdtf_description) => {
                    gdtf_descriptions.insert(fixture.gdtf_file_name.clone(), gdtf_description);
                }
                Err(err) => {
                    return Err(serde::de::Error::custom(format!(
                        "Failed to load GDTF description for fixture: {}",
                        err
                    )));
                }
            }
        }

        Ok(PatchList {
            fixtures,
            gdtf_descriptions,
        })
    }
}

fn load_gdtf_description(path: &str) -> Result<GdtfDescription, anyhow::Error> {
    let fixture_file = Assets
        .load(&path)
        .map_err(|e| anyhow!("Failed to load fixture file: {}", e))?;
    GdtfDescription::from_archive_bytes(&fixture_file)
        .map_err(|e| anyhow!("Failed to parse GDTF: {}", e.to_string()))
}

impl serde::Serialize for PatchList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.fixtures.serialize(serializer)
    }
}

impl PatchList {
    pub fn all_used_attributes(&self) -> Vec<&Attribute> {
        self.fixtures
            .iter()
            .flat_map(|f| {
                let fixture_type = self.fixture_type(f);
                fixture_type
                    .used_channels_for_mode(f.mode_index)
                    .iter()
                    .flat_map(|c| &c.logical_channels)
                    .map(|lc| lc.attribute(&fixture_type.attribute_definitions.attributes))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    pub fn fixture_type(&self, fixture: &Fixture) -> &FixtureType {
        &self
            .gdtf_descriptions
            .get(&fixture.gdtf_file_name)
            .expect("Fixture type not found")
            .fixture_type
    }

    pub fn fixture(&self, id: usize) -> Option<&Fixture> {
        self.fixtures.iter().find(|f| f.id == Some(id))
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Fixture {
    pub id: Option<FixtureId>,
    pub name: SharedString,
    pub gdtf_file_name: SharedString,
    pub mode_index: usize,
    pub patch: Option<DmxChannel>,
    pub dmx_values: Vec<u8>,
}

impl Fixture {
    pub fn new(
        name: SharedString,
        gdtf_file_name: SharedString,
        mode_index: usize,
        patch: Option<DmxChannel>,
    ) -> Self {
        Fixture {
            id: None,
            name,
            gdtf_file_name,
            mode_index,
            patch,
            dmx_values: vec![0; 512],
        }
    }

    pub fn get_dmx_value_with_offset(&self, offset: usize) -> u8 {
        self.dmx_values[offset - 1]
    }

    pub fn set_dmx_value_with_offset(&mut self, offset: usize, value: u8) {
        self.dmx_values[offset - 1] = value;
    }
}
