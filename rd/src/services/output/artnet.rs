use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use rd_artnet::FixedString;
use rd_dmx::{Channel, Multiverse};

use crate::project;

const REFRESH_RATE: u16 = 40;

#[derive(Default)]
pub struct ArtnetOutputService {
    config: project::artnet::ArtnetOutputConfig,

    node: RwLock<Option<rd_artnet::Node>>,
    service_notify_tx: RwLock<Option<flume::Sender<()>>>,
    multiverse: Arc<RwLock<Multiverse>>,
}

impl ArtnetOutputService {
    pub fn new(
        config: project::artnet::ArtnetOutputConfig,
        multiverse: Arc<RwLock<Multiverse>>,
    ) -> Self {
        Self { config, node: RwLock::new(None), service_notify_tx: RwLock::new(None), multiverse }
    }
}

impl rd_service::Delegate for ArtnetOutputService {
    type Error = anyhow::Error;
    type Data = ();

    fn on_start(&self) -> Result<(), Self::Error> {
        let (service_notify_tx, service_notify_rx) = flume::bounded(1);
        *self.service_notify_tx.write().unwrap() = Some(service_notify_tx);

        let mut pa_to_lu = HashMap::new();
        for instance in self.config.instances.iter() {
            pa_to_lu.insert(instance.port_address, instance.local_universe);
        }

        let dmx_provider = Arc::new({
            let multiverse = Arc::clone(&self.multiverse);
            move |artnet_universe: &mut rd_artnet::Universe,
                  port_address: rd_artnet::PortAddress| {
                let Some(local_universe) = pa_to_lu.get(&port_address).copied() else {
                    return Ok(());
                };

                let Some(universe) = multiverse.read().unwrap().universe(&local_universe).cloned()
                else {
                    return Ok(());
                };

                for channel in 0..512 {
                    let value = universe.value(&Channel::new_unchecked(channel + 1)).as_u8();
                    artnet_universe.set_channel(channel as usize, value)?;
                }

                Ok(())
            }
        });

        let mut node_config = rd_artnet::NodeConfig::new(
            FixedString::try_from_str("Radiant").expect("String should be less than 17 bytes"),
            FixedString::try_from_str("Radiant").expect("String should be less than 63 bytes"),
            dmx_provider,
        )
        .with_poll_reply_strategy(rd_artnet::PollReplyStrategy::Broadcast)
        .with_frame_scheduler(rd_artnet::FrameScheduler::External {
            refresh_rate: REFRESH_RATE,
            notifier: Arc::new(Box::new(move |notify_tx| {
                loop {
                    match service_notify_rx.recv() {
                        Ok(()) => {
                            let _ = notify_tx.send(());
                        }
                        Err(_) => break,
                    }
                }
            })),
        })
        .with_network_config(self.config.network.clone());

        for (ix, instance) in self.config.instances.iter().enumerate() {
            node_config.add_port(
                rd_artnet::PortConfig::new(rd_artnet::PortDirection::Output(instance.port_address))
                    .with_physical((ix + 1) as u8),
            );
        }

        *self.node.write().unwrap() = Some(rd_artnet::Node::new(node_config)?);

        Ok(())
    }

    fn on_frame(&self, _data: Self::Data) -> Result<(), Self::Error> {
        let service_notify_tx_guard = self.service_notify_tx.read().unwrap();
        if let Some(service_notify_tx) = service_notify_tx_guard.as_ref() {
            let _ = service_notify_tx.send(());
        }

        Ok(())
    }

    fn on_stop(&self) -> Result<(), Self::Error> {
        let _ = self.node.write().unwrap().take();
        Ok(())
    }
}
