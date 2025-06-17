use std::path::PathBuf;

use crate::backend::{patch::Patch, pipeline::Pipeline};

#[derive(Default)]
pub struct Show {
    /// The path at which the [Showfile] is saved.
    /// Will be `None` if it has not been saved yet.
    path: Option<PathBuf>,

    pub patch: Patch,

    /// The programmer contains WIP output data that can be saved to a preset.
    pub programmer: Pipeline,
}

impl Show {
    pub fn new(path: Option<PathBuf>) -> Self {
        Self { path, patch: Patch::default(), programmer: Pipeline::default() }
    }

    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }
}
