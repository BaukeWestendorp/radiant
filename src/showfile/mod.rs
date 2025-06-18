use std::path::{Path, PathBuf};

use crate::{error::Result, showfile::patch::Patch};

pub mod patch;

/// The showfile's file extension; 'rsf' (Radiant ShowFile).
pub const FILE_EXTENSION: &str = "rsf";

pub const RELATIVE_GDTF_FILE_FOLDER_PATH: &str = "gdtf_files";
pub const RELATIVE_PATCH_FILE_PATH: &str = "patch.yaml";

/// Represents the showfile that is saved on disk.
#[derive(Default)]
pub struct Showfile {
    path: Option<PathBuf>,

    pub patch: Patch,
}

impl Showfile {
    /// The path at which the [Showfile] is saved.
    /// Will be `None` if it has not been saved yet.
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

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
