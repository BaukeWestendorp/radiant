use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::num::NonZeroU32;
use std::path::Path;

use eyre::Context;
use gdtf::GdtfFile;
use gdtf::fixture_type::FixtureType;

use crate::comp::ShowfileComponent;
use crate::engine::Engine;
use crate::error::Result;

pub(crate) fn register(engine: &mut Engine) -> Result<()> {
    engine.register_component::<Patch>()?;
    Ok(())
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Patch {
    #[serde(skip)]
    gdtf_fixture_types: HashMap<GdtfFixtureTypeId, FixtureType>,

    fixtures: Vec<Fixture>,
}

impl Patch {
    pub fn fixture(&self, fid: impl Into<FixtureId>) -> Option<&Fixture> {
        let fid = fid.into();
        self.fixtures.iter().find(|f| f.fid == fid)
    }

    pub fn has_fixture(&self, fid: impl Into<FixtureId>) -> bool {
        let fid = fid.into();
        self.fixtures.iter().any(|f| f.fid == fid)
    }

    pub(crate) fn add_fixture(&mut self, fixture: Fixture) -> Result<()> {
        if !self.has_fixture(fixture.fid) {
            eyre::bail!("fixture with fixture id '{}' already exists", fixture.fid);
        }

        let Some(fixture_type) = self.gdtf_fixture_types.get(&fixture.gdtf_type_id) else {
            eyre::bail!("GDTF fixture type with id '{}' not found", fixture.fid);
        };

        if fixture_type.dmx_mode(&fixture.dmx_mode).is_none() {
            eyre::bail!(
                "DMX mode with name '{}' on fixture type '{}' not found",
                fixture.dmx_mode,
                fixture_type.long_name
            );
        }

        self.fixtures.push(fixture);

        Ok(())
    }
}

impl ShowfileComponent for Patch {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn relative_file_path() -> &'static str {
        "patch.yaml"
    }

    fn after_load_from_file(&mut self, showfile_path: &Path) -> Result<()> {
        const GDTF_FOLDER: &str = "gdtf_files";
        let path = showfile_path.join(GDTF_FOLDER);

        let entries = path.read_dir().wrap_err_with(|| {
            format!("failed to read gdtf_files folder at path: {}", path.display())
        })?;

        for entry in entries {
            let entry = entry.wrap_err_with(|| {
                format!("failed to read directory entry in folder: {}", path.display())
            })?;

            if !entry.file_name().as_os_str().to_string_lossy().ends_with(".gdtf") {
                continue;
            }

            let file = File::open(entry.path())
                .wrap_err_with(|| format!("failed to open gdtf file {}", entry.path().display()))?;
            let gdtf_file = GdtfFile::new(file)
                .wrap_err_with(|| format!("failed to read gdtf file {}", entry.path().display()))?;

            for fixture_type in gdtf_file.description.fixture_types {
                self.gdtf_fixture_types.insert(fixture_type.fixture_type_id, fixture_type);
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Fixture {
    pub name: String,
    pub fid: FixtureId,
    pub gdtf_type_id: GdtfFixtureTypeId,
    pub address: dmx::Address,
    pub dmx_mode: String,
}

impl Fixture {
    pub fn new(
        fid: FixtureId,
        gdtf_type_id: GdtfFixtureTypeId,
        address: dmx::Address,
        dmx_mode: String,
        name: String,
    ) -> Self {
        Self { fid, gdtf_type_id, address, dmx_mode, name }
    }
}

impl Fixture {
    pub fn gdtf_fixture_type<'a>(&self, patch: &'a Patch) -> &'a FixtureType {
        patch
            .gdtf_fixture_types
            .get(&self.gdtf_type_id)
            .expect("every fixture should have a valid GDTF Fixture Type")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FixtureId(pub NonZeroU32);

impl fmt::Display for FixtureId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type GdtfFixtureTypeId = uuid::Uuid;
