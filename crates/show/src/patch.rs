use crate::fixture::FixtureId;
use dmx::DmxAddress;
use gdtf::GdtfFile;
use gpui::SharedString;
use std::collections::HashMap;

#[derive(Clone, Default, serde::Serialize)]
pub struct Patch {
    #[serde(skip_serializing)]
    gdtf_descriptions: HashMap<SharedString, gdtf::Description>,
    fixtures: Vec<PatchedFixture>,
}

impl Patch {
    pub fn fixture(&self, id: FixtureId) -> Option<&PatchedFixture> {
        self.fixtures.iter().find(|f| f.id == id)
    }

    pub fn gdtf_description(&self, id: FixtureId) -> Option<&gdtf::Description> {
        let fixture = self.fixture(id)?;
        self.gdtf_descriptions.get(&fixture.gdtf_file_name)
    }

    pub fn patch_fixture(
        &mut self,
        id: FixtureId,
        dmx_address: DmxAddress,
        gdtf_file_name: SharedString,
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
                .patch_fixture(fixture.id(), fixture.dmx_address, fixture.gdtf_file_name)
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
    pub gdtf_file_name: SharedString,
}

impl PatchedFixture {
    pub fn id(&self) -> FixtureId {
        self.id
    }
}
