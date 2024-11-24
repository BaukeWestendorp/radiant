use crate::fixture::FixtureId;
use dmx::DmxAddress;
use gdtf::{dmx_mode::DmxMode, fixture_type::FixtureType, GdtfFile};
use gpui::SharedString;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct Patch {
    #[serde(skip_serializing)]
    gdtf_descriptions: HashMap<SharedString, gdtf::Description>,
    fixtures: Vec<PatchedFixture>,
}

impl Patch {
    pub fn fixtures(&self) -> &[PatchedFixture] {
        &self.fixtures
    }

    pub fn fixture(&self, id: FixtureId) -> Option<&PatchedFixture> {
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
        dmx_address: DmxAddress,
        gdtf_file_name: SharedString,
        dmx_mode: SharedString,
    ) -> anyhow::Result<()> {
        let path = dirs::cache_dir()
            .unwrap()
            .join("radiant")
            .join("gdtf_share_fixtures")
            .join(gdtf_file_name.to_string());
        log::debug!("Loading cached gdtf file {}", path.display());
        let file = std::fs::File::open(path)?;
        self.gdtf_descriptions
            .insert(gdtf_file_name.clone(), GdtfFile::new(file)?.description);

        let fixture = PatchedFixture {
            id,
            dmx_address,
            gdtf_file_name,
            dmx_mode,
        };
        self.fixtures.push(fixture);

        Ok(())
    }
}

impl<'de> serde::Deserialize<'de> for Patch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct IntermediatePatch {
            fixtures: Vec<PatchedFixture>,
        }

        let IntermediatePatch { fixtures } = IntermediatePatch::deserialize(deserializer)?;

        let mut patch = Patch::default();

        for fixture in fixtures {
            patch
                .patch_fixture(
                    fixture.id(),
                    fixture.dmx_address,
                    fixture.gdtf_file_name,
                    fixture.dmx_mode,
                )
                .map_err(|err| {
                    serde::de::Error::custom(format!("Failed to patch fixture: {err}"))
                })?;
        }

        Ok(patch)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatchedFixture {
    id: FixtureId,

    pub dmx_address: DmxAddress,
    gdtf_file_name: SharedString,
    dmx_mode: SharedString,
}

impl PatchedFixture {
    pub fn id(&self) -> FixtureId {
        self.id
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
        self.fixture_type(patch).dmx_mode(&self.dmx_mode).unwrap()
    }

    pub fn channel_offset_for_attribute<'p>(
        &self,
        attribute_name: &str,
        patch: &'p Patch,
    ) -> Option<&'p Vec<i32>> {
        let fixture_type = self.fixture_type(patch);

        for channel in &fixture_type.dmx_mode(&self.dmx_mode).unwrap().dmx_channels {
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

        return None;
    }
}
