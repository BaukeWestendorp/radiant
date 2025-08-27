use std::path::Path;
use std::{fs, io};

use eyre::Context;

use crate::error::Result;

pub trait ShowfileComponent: Sized + serde::Serialize + for<'de> serde::Deserialize<'de> {
    const RELATIVE_FILE_PATH: &str;

    /// Reads the implementing type from a YAML file at the given relative path
    /// inside the folder.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or parsed.
    fn read_from_showfile_folder(showfile_folder: &Path) -> Result<Self> {
        let path = showfile_folder.join(Self::RELATIVE_FILE_PATH);
        let file = fs::File::open(&path)
            .with_context(|| format!("failed to open file at '{}'", path.display()))?;
        let reader = io::BufReader::new(file);
        serde_yaml::from_reader(reader)
            .with_context(|| format!("failed to read YAML at '{}'", path.display()))
    }

    /// Writes the implementing type to a YAML file at the given absolute path.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be created or written to.
    fn write_to_file(&self, showfile_folder: &Path) -> Result<()> {
        let path = showfile_folder.join(Self::RELATIVE_FILE_PATH);
        let file = fs::File::create(&path)
            .with_context(|| format!("failed to create file at '{}'", path.display()))?;
        let writer = io::BufWriter::new(file);
        serde_yaml::to_writer(writer, &self)
            .with_context(|| format!("failed to write file at '{}'", path.display()))?;
        Ok(())
    }
}
