//! Showfile abstraction and persistent show configuration.
//!
//! This module defines the [Showfile] type and methods for loading, saving, and
//! representing showfiles on disk. A [Showfile] contains all persistent
//! configuration, including the patch, adapters, protocols, and initialization
//! commands. It can be loaded from disk and passed to an
//! [Engine][crate::engine::Engine] to produce a [Show][crate::show::Show] for
//! execution.

use std::path::{Path, PathBuf};

use crate::error::Result;

pub use objects::*;
pub use patch::*;
pub use protocols::*;

mod objects;
mod patch;
mod protocols;

/// The showfile's file extension; 'rsf' (Radiant ShowFile).
/// The file extension used for Radiant [Showfile]s.
pub const FILE_EXTENSION: &str = "rsf";

/// The relative path to the GDTF files folder within a [Showfile] directory.
pub const RELATIVE_GDTF_FILE_FOLDER_PATH: &str = "gdtf_files";
/// The relative path to the patch file within a [Showfile] directory.
pub const RELATIVE_PATCH_FILE_PATH: &str = "patch.yaml";
/// The relative path to the objects file within a [Showfile] directory.
pub const RELATIVE_OBJECTS_FILE_PATH: &str = "objects.yaml";
/// The relative path to the protocols file within a [Showfile] directory.
pub const RELATIVE_PROTOCOLS_FILE_PATH: &str = "protocols.yaml";

#[derive(Default)]
/// Represents a showfile that is saved on disk, containing all configuration
/// and state required to load a show, including patch, adapters,
/// protocols, and initialization commands.
pub struct Showfile {
    path: Option<PathBuf>,

    pub(crate) patch: Patch,
    pub(crate) objects: Objects,
    pub(crate) protocols: Protocols,
}

impl Showfile {
    /// Returns the path at which this [Showfile] is saved, or `None` if it has
    /// not been saved yet.
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    /// Loads a [Showfile] from the specified path. The path can refer to either
    /// a zipped or unzipped folder.
    pub fn load(path: &Path) -> Result<Self> {
        match path.extension() {
            Some(ext) if ext == FILE_EXTENSION => Self::load_zipped(path),
            Some(_) => {
                log::warn!(
                    "loading showfile with non-standard file extension: expected '.{FILE_EXTENSION}'"
                );
                Self::load_zipped(path)
            }
            None => Self::load_folder(path),
        }
    }

    /// Loads a [Showfile] from a zipped folder.
    pub fn load_zipped(_path: &Path) -> Result<Self> {
        todo!("opening zipped files is not yet implemented");
    }

    /// Loads a [Showfile] from an unzipped folder.
    pub fn load_folder(path: &Path) -> Result<Self> {
        let patch = Patch::read_from_file(&path.join(RELATIVE_PATCH_FILE_PATH))?;
        let objects = Objects::read_from_file(&path.join(RELATIVE_OBJECTS_FILE_PATH))?;
        let protocols = Protocols::read_from_file(&path.join(RELATIVE_PROTOCOLS_FILE_PATH))?;
        Ok(Self { path: Some(path.to_path_buf()), patch, protocols, objects })
    }
}
