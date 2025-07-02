use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use eyre::Context;

use crate::{
    Result,
    protocols::sacn,
    showfile::{SacnOutputType, SacnSourceConfiguration},
};

// FIXME: We should find a way to create a unique UUID for a device, without it changing over it's lifetime.
const CID: sacn::ComponentIdentifier = sacn::ComponentIdentifier::from_bytes([
    0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
]);

const SYNCHRONIZATION_ADDRESS: u16 = 0;
const FORCE_SYNCHRONIZATION: bool = false;

pub struct Protocols {
    sacn_sources: Vec<SacnSource>,
}

impl Protocols {
    pub fn new() -> Self {
        Self { sacn_sources: Vec::new() }
    }

    pub fn add_sacn_source(&mut self, config: &SacnSourceConfiguration) -> Result<()> {
        let ip = match config.r#type() {
            SacnOutputType::Unicast { destination_ip } => *destination_ip,
        };

        let local_universes = config.local_universes().to_vec();

        let source = Arc::new(
            sacn::Source::new(sacn::SourceConfig {
                cid: CID,
                name: config.name().to_string(),
                ip,
                port: sacn::DEFAULT_PORT,
                priority: config.priority(),
                preview_data: config.preview_data(),
                synchronization_address: SYNCHRONIZATION_ADDRESS,
                force_synchronization: FORCE_SYNCHRONIZATION,
            })
            .wrap_err("failed to create sACN source")?,
        );

        let thread_handle = thread::spawn({
            let source = source.clone();
            move || {
                source.start().expect("sACN source failed to start");
            }
        });

        self.sacn_sources.push(SacnSource {
            local_universes,
            thread_handle: Some(thread_handle),
            source,
        });

        Ok(())
    }

    pub fn update_dmx_output(&self, multiverse: &dmx::Multiverse) {
        let update_sacn_source = |source: &SacnSource| {
            source.source.clear_universes();
            for (id, universe) in multiverse.universes() {
                if source.local_universes.contains(id) {
                    let mut sacn_universe = sacn::Universe::new((*id).into());
                    sacn_universe.data_slots = universe.values().iter().map(|v| v.0).collect();
                    source.source.set_universe(sacn_universe);
                }
            }
        };

        for sacn_source in &self.sacn_sources {
            update_sacn_source(sacn_source);
        }
    }
}

impl Drop for Protocols {
    fn drop(&mut self) {
        for source in &mut self.sacn_sources {
            source.thread_handle.take();
        }
    }
}

struct SacnSource {
    local_universes: Vec<dmx::UniverseId>,
    thread_handle: Option<JoinHandle<()>>,
    source: Arc<sacn::Source>,
}
