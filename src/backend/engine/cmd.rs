use crate::{
    backend::patch::fixture::{DmxMode, FixtureId},
    dmx,
};

/// A [Command] is the interface between the engine and the backend.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    PatchFixture { id: FixtureId, address: dmx::Address, dmx_mode: DmxMode, gdtf_file_name: String },
}
