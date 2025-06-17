use std::{
    fs,
    path::{Path, PathBuf},
};

use eyre::{Context, ContextCompat};

use crate::{
    backend::{
        self,
        patch::fixture::{DmxMode, FixtureId},
        show::Show,
    },
    dmx,
    error::Result,
    showfile::patch::Patch,
};

pub mod patch;

/// The showfile's file extension; 'rsf' (Radiant ShowFile).
pub const FILE_EXTENSION: &str = "rsf";

pub const RELATIVE_GDTF_FILE_FOLDER_PATH: &str = "gdtf_files";
pub const RELATIVE_PATCH_FILE_PATH: &str = "patch.yaml";

/// Represents the showfile that is saved on disk.
#[derive(Default, facet::Facet)]
pub struct Showfile {
    /// The path at which the [Showfile] is saved.
    /// Will be `None` if it has not been saved yet.
    path: Option<PathBuf>,

    pub patch: Patch,
}

impl Showfile {
    /// Loads a [Showfile] from a path. It can be either a zipped folder, or an unzipped folder.
    pub fn load(path: &Path) -> Result<Self> {
        match path.extension() {
            Some(ext) if ext == FILE_EXTENSION => Self::load_zipped(path),
            Some(_) => {
                log::warn!(
                    "Loading showfile with non-standard file extension: expected '.{FILE_EXTENSION}'"
                );
                Self::load_zipped(path)
            }
            None => Self::load_folder(path),
        }
    }

    /// Loads a [Showfile] from a zipped folder.
    pub fn load_zipped(_path: &Path) -> Result<Self> {
        todo!("Opening zipped files is not yet implemented");
    }

    /// Loads a [Showfile] from an unzipped folder.
    pub fn load_folder(path: &Path) -> Result<Self> {
        let patch = Patch::read_from_file(path.join(RELATIVE_PATCH_FILE_PATH))?;

        Ok(Self { path: Some(path.to_path_buf()), patch })
    }
}

impl Showfile {
    pub fn into_show(self) -> Result<backend::show::Show> {
        let mut patch = backend::patch::Patch::default();

        for fixture in &self.patch.fixtures {
            let fixture_id = FixtureId(fixture.id);

            let address = dmx::Address::new(
                dmx::UniverseId::new(fixture.universe)?,
                dmx::Channel::new(fixture.channel)?,
            );

            let dmx_mode = DmxMode::new(fixture.dmx_mode.clone());

            let gdtf_file_name = self.patch.gdtf_files.get(fixture.gdtf_file_index).context("Failed to generate patch: Tried to reference GDTF file index that is out of bounds")?.to_string();

            let showfile_path = match &self.path {
                Some(path) => path,
                None => {
                    todo!("Support creating new showfiles and defining their temporary location")
                }
            };
            let gdtf_file_path = Path::new(&showfile_path)
                .join(RELATIVE_GDTF_FILE_FOLDER_PATH)
                .join(&gdtf_file_name);
            let gdtf_file = fs::File::open(gdtf_file_path).context("Failed to open GDTF file")?;
            let fixture_type = &gdtf::GdtfFile::new(gdtf_file)
                .context("Failed to read GDTF file")?
                .description
                .fixture_types[0];

            patch
                .patch_fixture(fixture_id, address, dmx_mode, gdtf_file_name, fixture_type)
                .context("Failed to patch fixture into Show")?;
        }

        Ok(Show { patch })
    }
}
