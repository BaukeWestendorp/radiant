const DESCRIPTION_FILE_NAME: &str = "description.xml";

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

mod description;
mod source;

pub use description::*;

use source::{ArchiveBytesSource, ArchiveSource, FolderSource, SourceLoader as _};

/// Representation of an GDTF bundle.
///
/// This stays close to the serialized `Description.xml` and the
/// files contained in the bundle (folder/zip).
pub struct Bundle {
    description: GdtfDescription,
    resources: HashMap<PathBuf, Vec<u8>>,
    path: Option<PathBuf>,
}

impl Bundle {
    pub fn from_folder(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        FolderSource::new(path.clone()).load_bundle()
    }

    pub fn from_archive(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        ArchiveSource::new(path).load_bundle()
    }

    pub fn from_archive_bytes(bytes: &[u8]) -> Self {
        ArchiveBytesSource::new(bytes).load_bundle()
    }

    pub fn description(&self) -> &GdtfDescription {
        &self.description
    }

    pub fn resources(&self) -> &HashMap<PathBuf, Vec<u8>> {
        &self.resources
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
}

pub(crate) trait FromBundle {
    type Source;

    fn from_bundle(source: &Self::Source, bundle: &Bundle) -> Self;
}
