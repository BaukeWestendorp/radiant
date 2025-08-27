use crate::show::ObjectContainer;
use crate::showfile::ShowfileComponent;

/// A collection of lighting control objects loaded from configuration.
///
/// The `Objects` struct contains all the major elements used in the lighting
/// control system, such as executors, sequences, cues, fixture groups, and
/// presets.
#[derive(Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Objects {
    #[serde(default)]
    pub(crate) object_container: ObjectContainer,
}

impl ShowfileComponent for Objects {
    const RELATIVE_FILE_PATH: &str = "objects.yaml";
}
