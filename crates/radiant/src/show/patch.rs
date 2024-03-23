use std::collections::HashMap;

use assets::{AssetSource, Assets};
use dmx::DmxChannel;
use gdtf::{Attribute, FixtureType, GdtfDescription, Guid};
use gpui::SharedString;

pub type FixtureId = usize;

#[derive(Debug, Clone, Default)]
pub struct PatchList {
    pub fixtures: Vec<Fixture>,

    gdtf_descriptions: HashMap<Guid, GdtfDescription>,
}

impl<'de> serde::Deserialize<'de> for PatchList {
    fn deserialize<D>(deserializer: D) -> Result<PatchList, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, serde::Deserialize)]
        struct IntermediatePatchList {
            fixtures: Vec<Fixture>,
        }
        let intermediate: IntermediatePatchList = serde::Deserialize::deserialize(deserializer)?;

        let mut fixtures = intermediate.fixtures;
        let mut gdtf_descriptions = HashMap::new();
        for fixture in fixtures.iter_mut() {
            let path = format!("fixtures/{}", fixture.gdtf_file_name);
            match load_gdtf_description(path.as_str()) {
                Ok(gdtf_description) => {
                    if fixture.channel_values.is_empty() {
                        fixture.set_default_channel_values(&gdtf_description.fixture_type);
                    }

                    gdtf_descriptions
                        .insert(gdtf_description.fixture_type.id.clone(), gdtf_description);
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
                let fixture_type = self.get_fixture_type(&f.fixture_type_id);
                let used_channels = fixture_type.used_channels_for_mode(f.mode_index);

                used_channels
                    .iter()
                    .flat_map(|c| &c.logical_channels)
                    .map(|lc| lc.attribute(&fixture_type.attribute_definitions.attributes))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    pub fn get_fixture_type(&self, id: &Guid) -> &FixtureType {
        &self.gdtf_descriptions.get(id).unwrap().fixture_type
    }

    pub fn fixture(&self, id: usize) -> Option<&Fixture> {
        self.fixtures.iter().find(|f| f.id == Some(id))
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Fixture {
    pub id: Option<FixtureId>,
    pub name: SharedString,
    pub fixture_type_id: Guid,
    pub(self) gdtf_file_name: String,
    pub mode_index: usize,
    pub channel: Option<DmxChannel>,
    pub channel_values: HashMap<String, Vec<u8>>,
}

impl Fixture {
    pub fn new(
        name: SharedString,
        fixture_type_id: Guid,
        gdtf_file_name: String,
        mode_index: usize,
        channel: Option<DmxChannel>,
    ) -> Self {
        Fixture {
            id: None,
            name,
            fixture_type_id,
            gdtf_file_name,
            mode_index,
            channel,
            channel_values: HashMap::new(),
        }
    }

    pub fn dmx_values(&self, patch_list: &PatchList) -> Vec<u8> {
        let attributes = patch_list
            .get_fixture_type(&self.fixture_type_id)
            .attribute_definitions
            .attributes
            .clone();

        let mut values = vec![];
        patch_list
            .get_fixture_type(&self.fixture_type_id)
            .used_channels_for_mode(self.mode_index)
            .iter()
            .for_each(|channel| {
                let Some(offset) = channel.offset.clone() else {
                    return;
                };

                if offset.is_empty() {
                    return;
                }

                // Make sure we have enough space in the values vector
                let last_offset = usize::try_from(*offset.last().unwrap()).unwrap_or(0);
                if last_offset > values.len() {
                    values.resize(last_offset, 0);
                }

                let attribute = channel
                    .logical_channels
                    .first()
                    .unwrap()
                    .attribute(&attributes)
                    .clone();
                let value = self.channel_value_for_attribute(&attribute.name).unwrap();

                for (i, o) in offset.iter().enumerate() {
                    values[*o as usize - 1] = value[i];
                }
            });
        values
    }

    pub fn channel_value_for_attribute(&self, name: &str) -> Option<&Vec<u8>> {
        self.channel_values.get(name)
    }

    pub fn set_default_channel_values(&mut self, fixture_type: &FixtureType) {
        let default_channel_values =
            fixture_type.dmx_modes[self.mode_index].default_channel_values();
        self.channel_values = default_channel_values;
    }
}
