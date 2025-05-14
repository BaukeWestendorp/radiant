use crate::show::dmx_io::{DmxIoSettings, SacnOutputType};
use anyhow::Context;
use gpui::*;
use sacn::Universe;

use std::sync::Arc;

const CID: sacn::ComponentIdentifier = sacn::ComponentIdentifier::from_bytes([
    0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
]);

pub struct DmxIo {
    pub multiverse: Entity<dmx::Multiverse>,

    sacn_sources: Vec<Arc<SacnSource>>,
}

impl DmxIo {
    pub fn new(
        multiverse: Entity<dmx::Multiverse>,
        settings: &DmxIoSettings,
        cx: &App,
    ) -> anyhow::Result<Self> {
        let sacn_sources = settings
            .sacn
            .sources
            .iter()
            .map(|s| -> anyhow::Result<_> {
                const SYNCHRONIZATION_ADDRESS: u16 = 0;
                const FORCE_SYNCHRONIZATION: bool = false;
                let s = s.read(cx);

                let ip = match s.r#type {
                    SacnOutputType::Unicast { destination_ip } => {
                        destination_ip.expect("destination ip required")
                    }
                };

                let config = sacn::SourceConfig {
                    cid: CID,
                    name: s.name.to_string(),
                    ip,
                    port: sacn::DEFAULT_PORT,
                    priority: s.priority,
                    preview_data: s.preview_data,
                    synchronization_address: SYNCHRONIZATION_ADDRESS,
                    force_synchronization: FORCE_SYNCHRONIZATION,
                };
                let source = sacn::Source::new(config).context("create sACN source")?;
                source.set_universe(Universe::new(s.destination_universe));

                let local_universes = s
                    .local_universes
                    .iter()
                    .map(|u| dmx::UniverseId::new(*u))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Arc::new(SacnSource { local_universes, source }))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { multiverse, sacn_sources })
    }

    pub fn start(&self, cx: &mut App) {
        cx.observe(&self.multiverse, {
            let sacn_sources = self.sacn_sources.clone();
            move |multiverse, cx| {
                for sacn_source in &sacn_sources {
                    sacn_source.source.clear_universes();
                    for (id, universe) in multiverse.read(cx).universes() {
                        if sacn_source.local_universes.contains(id) {
                            let mut sacn_universe = sacn::Universe::new((*id).into());
                            sacn_universe.data_slots =
                                universe.values().iter().map(|v| v.0).collect();
                            sacn_source.source.set_universe(sacn_universe);
                        }
                    }
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
}

struct SacnSource {
    local_universes: Vec<dmx::UniverseId>,
    source: sacn::Source,
}
