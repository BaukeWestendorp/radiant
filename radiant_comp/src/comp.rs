use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::error::Result;

pub trait ShowfileComponent: Send + Sync + 'static {
    fn as_any(&self) -> &dyn std::any::Any;

    fn relative_file_path() -> &'static str
    where
        Self: Sized;

    fn read_from_showfile(showfile_path: &Path) -> Result<Self>
    where
        Self: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let file_path = showfile_path.join(Self::relative_file_path());
        let file = File::open(&file_path)?;
        let reader = BufReader::new(file);
        let value = serde_yaml::from_reader(reader)?;
        Ok(value)
    }

    fn after_load_from_file(&mut self, _showfile_path: &Path) -> Result<()> {
        Ok(())
    }
}
