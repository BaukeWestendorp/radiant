use crate::show::{ProtocolSettings, SacnOutputType, SacnSourceSettings};
use anyhow::Context as _;
use gpui::{App, Entity, prelude::*};
use std::sync::Arc;

// FIXME: We should find a way to create a unique UUID for a device, without it changing over it's lifetime.
const CID: sacn::ComponentIdentifier = sacn::ComponentIdentifier::from_bytes([
    0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
]);

pub struct SacnOutputProtocolManager {
    /// The DMX data to send.
    multiverse: Entity<dmx::Multiverse>,

    /// Managed sACN sources.
    sacn_sources: Vec<Arc<SacnSource>>,
}

impl SacnOutputProtocolManager {
    /// Creates a new [SacnOutputProtocolManager].
    pub fn new(
        settings: &ProtocolSettings,
        multiverse: Entity<dmx::Multiverse>,
        cx: &App,
    ) -> anyhow::Result<Self> {
        let mut this = Self { multiverse, sacn_sources: Vec::new() };

        for sacn_source in &settings.sacn.sources {
            this.add_sacn_source(sacn_source.read(cx))?;
        }

        Ok(this)
    }

    /// Starts all sACN sources.
    pub fn start(&self, cx: &mut App) {
        cx.observe(&self.multiverse, {
            let sacn_sources = self.sacn_sources.clone();
            move |multiverse, cx| {
                let update_sacn_source = |sacn_source: &SacnSource| {
                    sacn_source.source.clear_universes();
                    for (id, universe) in multiverse.read(cx).universes() {
                        if sacn_source.local_universes.contains(id) {
                            let mut sacn_universe = sacn::Universe::new((*id).into());
                            sacn_universe.data_slots =
                                universe.values().iter().map(|v| v.0).collect();
                            sacn_source.source.set_universe(sacn_universe);
                        }
                    }
                };

                for sacn_source in &sacn_sources {
                    update_sacn_source(sacn_source);
                }
            }
        })
        .detach();

        for sacn_source in self.sacn_sources.clone() {
            cx.background_spawn(async move {
                if let Err(err) = sacn_source.source.start() {
                    log::error!("Failed to start sACN source: {err}")
                }
            })
            .detach();
        }
    }

    /// Adds a new sACN source.
    pub fn add_sacn_source(&mut self, source_settings: &SacnSourceSettings) -> anyhow::Result<()> {
        const SYNCHRONIZATION_ADDRESS: u16 = 0;
        const FORCE_SYNCHRONIZATION: bool = false;

        let ip = match source_settings.r#type {
            SacnOutputType::Unicast { destination_ip } => {
                destination_ip.expect("destination ip required")
            }
        };

        let config = sacn::SourceConfig {
            cid: CID,
            name: source_settings.name.to_string(),
            ip,
            port: sacn::DEFAULT_PORT,
            priority: source_settings.priority,
            preview_data: source_settings.preview_data,
            synchronization_address: SYNCHRONIZATION_ADDRESS,
            force_synchronization: FORCE_SYNCHRONIZATION,
        };
        let source = sacn::Source::new(config).context("create sACN source")?;
        source.set_universe(sacn::Universe::new(source_settings.destination_universe));

        let local_universes = source_settings
            .local_universes
            .iter()
            .map(|u| dmx::UniverseId::new(*u))
            .collect::<Result<Vec<_>, _>>()?;

        self.sacn_sources.push(Arc::new(SacnSource { local_universes, source }));

        Ok(())
    }
}

struct SacnSource {
    local_universes: Vec<dmx::UniverseId>,
    source: sacn::Source,
}
