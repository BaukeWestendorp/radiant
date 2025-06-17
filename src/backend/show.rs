use std::path::PathBuf;

use crate::backend::patch::Patch;

#[derive(Default)]
pub struct Show {
    /// The path at which the [Showfile] is saved.
    /// Will be `None` if it has not been saved yet.
    path: Option<PathBuf>,

    pub patch: Patch,
}

impl Show {
    pub fn new(path: Option<PathBuf>) -> Self {
        Self { path, patch: Patch::default() }
    }

    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }
}
