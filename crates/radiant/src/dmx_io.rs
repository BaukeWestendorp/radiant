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
    sacn_sources: Vec<Arc<sacn::Source>>,
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

                Ok(Arc::new(source))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { sacn_sources })
    }

    pub fn start(&mut self, cx: &mut App) {
        for sacn_source in self.sacn_sources.clone() {
            cx.background_spawn(async move {
                if let Err(err) = sacn_source.start() {
                    log::error!("Failed to start sACN source: {err}")
                }
            })
            .detach();
        }
    }
}

impl Global for DmxIo {}
