use anyhow::Context;
use gdtf::{GdtfFile, dmx_mode::DmxMode, fixture_type::FixtureType};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize)]
pub struct Patch {
    fixtures: Vec<Fixture>,
    #[serde(skip)]
    gdtf_descriptions: HashMap<String, gdtf::Description>,
}

impl Patch {
    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }

    pub fn fixture(&self, id: FixtureId) -> Option<&Fixture> {
        self.fixtures.iter().find(|f| f.id == id)
    }

    pub fn gdtf_description(&self, id: FixtureId) -> Option<&gdtf::Description> {
        let fixture = self.fixture(id)?;
        let description = self
            .gdtf_descriptions
            .get(&fixture.gdtf_file_name)
            .expect("A GDTF Description should exist for every fixture");
        Some(description)
    }

    pub fn patch_fixture(
        &mut self,
        id: FixtureId,
        address: dmx::Address,
        gdtf_file_name: String,
        dmx_mode: String,
    ) -> anyhow::Result<()> {
        let path = dirs::cache_dir()
            .expect("should get cache directory")
            .join("radiant")
            .join("gdtf_share_fixtures")
            .join(&gdtf_file_name);
        log::debug!("Loading cached gdtf file {}", path.display());
        let file = std::fs::File::open(&path)
            .with_context(|| format!("Could not open cached gdtf file at {:?}", path.display()))?;

        self.gdtf_descriptions.insert(
            gdtf_file_name.clone(),
            GdtfFile::new(file).context("Could not create new GdtfFile")?.description,
        );

        let fixture = Fixture { id, address, gdtf_file_name, dmx_mode };
        self.fixtures.push(fixture);

        Ok(())
    }
}

impl<'de> serde::Deserialize<'de> for Patch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut this = Self { fixtures: Vec::new(), gdtf_descriptions: HashMap::new() };

        #[derive(serde::Deserialize)]
        struct Intermediate {
            fixtures: Vec<Fixture>,
        }

        let intermediate = Intermediate::deserialize(deserializer)?;

        for fixture in intermediate.fixtures {
            this.patch_fixture(
                fixture.id,
                fixture.address,
                fixture.gdtf_file_name,
                fixture.dmx_mode,
            )
            .map_err(serde::de::Error::custom)?;
        }

        Ok(this)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FixtureId(pub u32);

impl From<u32> for FixtureId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl From<FixtureId> for u32 {
    fn from(id: FixtureId) -> Self {
        id.0
    }
}

impl std::str::FromStr for FixtureId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl std::fmt::Display for FixtureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Fixture {
    id: FixtureId,
    address: dmx::Address,
    gdtf_file_name: String,
    dmx_mode: String,
}

impl Fixture {
    pub fn address(&self) -> &dmx::Address {
        &self.address
    }

    pub fn fixture_type<'p>(&self, patch: &'p Patch) -> &'p FixtureType {
        patch
            .gdtf_description(self.id)
            .expect("A GDTF Description should exist for every fixture")
            .fixture_types
            .first()
            .unwrap()
    }

    pub fn dmx_mode<'p>(&self, patch: &'p Patch) -> &'p DmxMode {
        self.fixture_type(patch).dmx_mode(&self.dmx_mode).unwrap_or_else(|| {
            panic!(
                "Invalid DMX Mode: {}. valid modes are: [{:?}]",
                self.dmx_mode,
                self.fixture_type(patch)
                    .dmx_modes
                    .iter()
                    .filter_map(|m| m.name.as_ref().map(|n| n.to_string()))
            )
        })
    }

    pub fn channel_offset_for_attr<'p>(
        &self,
        attribute_name: &str,
        patch: &'p Patch,
    ) -> Option<&'p Vec<i32>> {
        let fixture_type = self.fixture_type(patch);

        for channel in &self.dmx_mode(patch).dmx_channels {
            let (logical_channel, _) = channel.initial_function().unwrap();

            if logical_channel
                .attribute(fixture_type)
                .unwrap()
                .name
                .as_deref()
                .is_some_and(|name| name == attribute_name)
            {
                return channel.offset.as_ref();
            }
        }

        None
    }
}
