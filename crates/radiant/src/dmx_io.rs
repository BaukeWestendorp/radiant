use anyhow::Context;
use gpui::*;
use network_interface::{NetworkInterface, NetworkInterfaceConfig as _};
use sacn::Universe;
use show::dmx_io::DmxIoSettings;
use std::sync::Arc;

const CID: sacn::ComponentIdentifier = sacn::ComponentIdentifier::from_bytes([
    0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
]);

pub struct DmxIo {
    pub multiverse: dmx::Multiverse,

    sacn_sources: Vec<Arc<SacnSource>>,
}

impl DmxIo {
    pub fn new(settings: &DmxIoSettings) -> anyhow::Result<Self> {
        let interfaces = NetworkInterface::show().context("get network interfaces")?;

        let interface = interfaces
            .iter()
            .find(|i| i.name == settings.interface.name)
            .or_else(|| {
                let new_if = interfaces.iter().find(|i| i.addr.len() > 0)?;
                log::warn!(
                    "Could not find network interface with name '{}'. using '{}' instead",
                    settings.interface.name,
                    new_if.name
                );
                Some(new_if)
            })
            .context("no network interface (with an available address) not found")?;

        let sacn_sources = settings
            .sacn
            .sources
            .iter()
            .map(|s| -> anyhow::Result<_> {
                const SYNCHRONIZATION_ADDRESS: u16 = 0;
                const FORCE_SYNCHRONIZATION: bool = false;

                let config = sacn::SourceConfig {
                    cid: CID,
                    name: s.name.to_string(),
                    ip: interface.addr.first().unwrap().ip(),
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

        Ok(Self { multiverse: dmx::Multiverse::new(), sacn_sources })
    }

    pub fn start(cx: &mut App) {
        cx.observe_global::<Self>(|cx| {
            Self::update_sacn_sources_from_multiverse(cx);
        })
        .detach();

        let this = Self::global(cx);
        for sacn_source in this.sacn_sources.clone() {
            cx.background_spawn(async move {
                if let Err(err) = sacn_source.source.start() {
                    log::error!("Failed to start sACN source: {err}")
                }
            })
            .detach();
        }
    }

    fn update_sacn_sources_from_multiverse(cx: &App) {
        let this = Self::global(cx);
        for sacn_source in &this.sacn_sources {
            sacn_source.source.clear_universes();
            for (id, universe) in this.multiverse.universes() {
                if sacn_source.local_universes.contains(id) {
                    let mut sacn_universe = sacn::Universe::new((*id).into());
                    sacn_universe.data_slots = universe.values().iter().map(|v| v.0).collect();
                    sacn_source.source.set_universe(sacn_universe);
                }
            }
        }
    }
}

impl Global for DmxIo {}

struct SacnSource {
    local_universes: Vec<dmx::UniverseId>,
    source: sacn::Source,
}
