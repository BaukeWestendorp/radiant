use std::collections::HashMap;

use dmx::DmxAddress;
use gdtf::{dmx_mode::DmxMode, fixture_type::FixtureType, GdtfFile};

use crate::showfile;

super::asset_id!(pub FixtureId);

#[derive(Debug)]
pub struct Patch {
    fixtures: Vec<Fixture>,
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
        dmx_address: DmxAddress,
        gdtf_file_name: String,
        dmx_mode: String,
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

        let fixture = Fixture {
            id,
            dmx_address,
            gdtf_file_name,
            dmx_mode,
        };
        self.fixtures.push(fixture);

        Ok(())
    }
}

impl Patch {
    pub fn try_from_showfile(patch: showfile::Patch) -> anyhow::Result<Self> {
        let mut this = Self {
            fixtures: Vec::new(),
            gdtf_descriptions: HashMap::new(),
        };

        for fixture in patch.fixtures {
            this.patch_fixture(
                fixture.id.into(),
                fixture.dmx_address,
                fixture.gdtf_file_name,
                fixture.dmx_mode,
            )?;
        }

        Ok(this)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fixture {
    id: FixtureId,
    dmx_address: dmx::DmxAddress,
    gdtf_file_name: String,
    dmx_mode: String,
}

impl Fixture {
    pub fn id(&self) -> FixtureId {
        self.id
    }

    pub fn dmx_address(&self) -> &dmx::DmxAddress {
        &self.dmx_address
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
        self.fixture_type(patch)
            .dmx_mode(&self.dmx_mode)
            .unwrap_or_else(|| {
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

    pub fn channel_offset_for_attribute<'p>(
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
