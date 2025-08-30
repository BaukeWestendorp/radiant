use crate::show::FixtureTypeId;
use crate::showfile::ShowfileComponent;

/// Represents the patch configuration for a show, including all information
/// about the mapping of fixtures to DMX universes and their associated GDTF
/// files.
///
/// The [Patch] contains a list of GDTF file names and all [Fixture]s that are
/// mapped in the show. It is responsible for describing how logical fixtures
/// are assigned to physical DMX addresses and universes.
#[derive(Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Patch {
    pub(crate) fixtures: Vec<Fixture>,
}

impl ShowfileComponent for Patch {
    const RELATIVE_FILE_PATH: &str = "patch.yaml";
}

/// Represents a single fixture mapped in the [Patch].
///
/// A [Fixture] describes the logical-to-physical mapping for a fixture,
/// including its unique id, the index of its GDTF file, DMX universe and
/// channel, and the DMX mode used for addressing.
#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Fixture {
    pub(crate) fid: u32,
    pub(crate) fixture_type_id: FixtureTypeId,
    pub(crate) universe: u16,
    pub(crate) channel: u16,
    pub(crate) dmx_mode: String,
}
