use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{Command, Result};

pub mod adapters;
pub mod patch;
pub mod protocols;

pub use adapters::*;
pub use patch::*;
pub use protocols::*;

/// The showfile's file extension; 'rsf' (Radiant ShowFile).
pub const FILE_EXTENSION: &str = "rsf";

pub const RELATIVE_GDTF_FILE_FOLDER_PATH: &str = "gdtf_files";
pub const RELATIVE_PATCH_FILE_PATH: &str = "patch.yaml";
pub const RELATIVE_ADAPTERS_FILE_PATH: &str = "adapters.yaml";
pub const RELATIVE_PROTOCOLS_FILE_PATH: &str = "protocols.yaml";
pub const RELATIVE_INIT_COMMANDS_FILE_PATH: &str = "init_commands.rcs";

/// Represents the showfile that is saved on disk.
#[derive(Default)]
pub struct Showfile {
    path: Option<PathBuf>,

    patch: Patch,
    adapters: Adapters,
    protocols: Protocols,
    init_commands: Vec<Command>,
}

impl Showfile {
    /// The path at which the [Showfile] is saved.
    /// Will be `None` if it has not been saved yet.
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    pub fn patch(&self) -> &Patch {
        &self.patch
    }

    pub fn adapters(&self) -> &Adapters {
        &self.adapters
    }

    pub fn protocols(&self) -> &Protocols {
        &self.protocols
    }

    pub fn init_commands(&self) -> &[Command] {
        &self.init_commands
    }

    /// Loads a [Showfile] from a path. It can be either a zipped folder, or an unzipped folder.
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
        let adapters = Adapters::read_from_file(&path.join(RELATIVE_ADAPTERS_FILE_PATH))?;
        let protocols = Protocols::read_from_file(&path.join(RELATIVE_PROTOCOLS_FILE_PATH))?;
        let init_commands = load_init_commands(&path.join(RELATIVE_INIT_COMMANDS_FILE_PATH))?;
        Ok(Self { path: Some(path.to_path_buf()), patch, adapters, protocols, init_commands })
    }
}

fn load_init_commands(path: &Path) -> Result<Vec<Command>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = std::fs::read_to_string(path)?;
    let commands = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| Command::from_str(line))
        .collect::<Result<Vec<_>>>()?;

    Ok(commands)
}
