pub(super) mod builder;
mod id;

use std::sync::Arc;

pub use id::*;

use crate::{
    dmx,
    mvr_gdtf::gdtf::{Gdtf, Name, NodePath, dmx::DmxMode},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Fixture {
    id: FixtureId,
    name: String,
    dmx_address: dmx::Address,

    gdtf: Arc<Gdtf>,
    dmx_mode: Name,
    channel_functions: Vec<NodePath>,
    geometry_dmx_offset: u32,

    child_ids: Vec<FixtureId>,
}

impl Fixture {
    pub fn id(&self) -> FixtureId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn dmx_address(&self) -> dmx::Address {
        self.dmx_address
    }

    pub fn dmx_mode(&self) -> &DmxMode {
        self.gdtf()
            .dmx_mode(&self.dmx_mode)
            .expect("Fixture should have a valid DMX mode name for it's GDTF")
    }

    pub fn gdtf(&self) -> &Gdtf {
        &self.gdtf
    }

    pub fn channel_functions(&self) -> &[NodePath] {
        &self.channel_functions
    }

    pub fn geometry_dmx_offset(&self) -> u32 {
        self.geometry_dmx_offset
    }

    pub fn child_ids(&self) -> &[FixtureId] {
        &self.child_ids
    }
}
