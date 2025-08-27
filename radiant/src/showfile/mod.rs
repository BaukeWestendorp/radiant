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
use crate::show::Show;
use crate::showfile::objects::Objects;
use crate::showfile::patch::Patch;
use crate::showfile::protocols::Protocols;

pub mod component;
mod objects;
mod patch;
mod protocols;

pub use component::*;

/// The showfile's file extension; 'rsf' (Radiant ShowFile).
/// The file extension used for Radiant [Showfile]s.
pub const FILE_EXTENSION: &str = "rsf";

/// The relative path to the GDTF files folder within a [Showfile] directory.
pub const RELATIVE_GDTF_FILE_FOLDER_PATH: &str = "gdtf_files";

/// Represents a showfile that is saved on disk, containing all configuration
/// and state required to load a show, including patch, adapters,
/// protocols, and initialization commands.
#[derive(Default)]
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
    pub fn load_folder(showfile_folder: &Path) -> Result<Self> {
        let patch = Patch::read_from_showfile_folder(showfile_folder)?;
        let objects = Objects::read_from_showfile_folder(showfile_folder)?;
        let protocols = Protocols::read_from_showfile_folder(showfile_folder)?;
        Ok(Self { path: Some(showfile_folder.to_path_buf()), patch, protocols, objects })
    }

    pub fn save(&self) -> Result<()> {
        let Some(path) = &self.path else {
            eyre::bail!("Showfile path is not set");
        };

        self.patch.write_to_file(&path)?;
        self.objects.write_to_file(&path)?;
        self.protocols.write_to_file(&path)?;

        Ok(())
    }
}

impl From<&Show> for Showfile {
    fn from(show: &Show) -> Self {
        let objects = Objects { object_container: show.objects.clone() };

        let mut patch = Patch::default();
        for fixture in show.patch().fixtures() {
            patch.fixtures.push(patch::Fixture {
                fid: fixture.fid().into(),
                gdtf_type_id: *fixture.fixture_type_id(),
                universe: fixture.address().universe.into(),
                channel: fixture.address().channel.into(),
                dmx_mode: fixture.dmx_mode.clone(),
            });
        }

        let protocols = Protocols { protocol_config: show.protocol_config.clone() };

        Self { path: show.path().cloned(), objects, patch, protocols }
    }
}
