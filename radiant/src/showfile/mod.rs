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
use crate::show::{
    Executor, Group, ObjectKind, PresetBeam, PresetColor, PresetControl, PresetDimmer, PresetFocus,
    PresetGobo, PresetPosition, PresetShapers, PresetVideo, Sequence, Show,
};

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
#[derive(Debug)]
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

impl From<&Show> for Showfile {
    fn from(show: &Show) -> Self {
        let mut objects = Objects::default();
        for (_, object) in show.objects().iter() {
            match object.kind() {
                ObjectKind::Group => objects.groups.push(object.as_impl::<Group>().clone()),
                ObjectKind::Executor => {
                    objects.executors.push(object.as_impl::<Executor>().clone())
                }
                ObjectKind::Sequence => {
                    objects.sequences.push(object.as_impl::<Sequence>().clone())
                }
                ObjectKind::PresetDimmer => {
                    objects.dimmer_presets.push(object.as_impl::<PresetDimmer>().clone())
                }
                ObjectKind::PresetPosition => {
                    objects.position_presets.push(object.as_impl::<PresetPosition>().clone())
                }
                ObjectKind::PresetGobo => {
                    objects.gobo_presets.push(object.as_impl::<PresetGobo>().clone())
                }
                ObjectKind::PresetColor => {
                    objects.color_presets.push(object.as_impl::<PresetColor>().clone())
                }
                ObjectKind::PresetBeam => {
                    objects.beam_presets.push(object.as_impl::<PresetBeam>().clone())
                }
                ObjectKind::PresetFocus => {
                    objects.focus_presets.push(object.as_impl::<PresetFocus>().clone())
                }
                ObjectKind::PresetControl => {
                    objects.control_presets.push(object.as_impl::<PresetControl>().clone())
                }
                ObjectKind::PresetShapers => {
                    objects.shapers_presets.push(object.as_impl::<PresetShapers>().clone())
                }
                ObjectKind::PresetVideo => {
                    objects.video_presets.push(object.as_impl::<PresetVideo>().clone())
                }
            }
        }

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
