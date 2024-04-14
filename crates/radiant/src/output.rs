use std::time::Duration;

use anyhow::Result;
use dmx::DmxOutput;
use gpui::{AppContext, BorrowAppContext, Global};

use crate::showfile::ShowfileManager;

const DMX_OUTPUT_RATE: Duration = Duration::from_millis(1000 / 40);

pub struct DmxOutputManager {
    protocols: Vec<Box<dyn DmxProtocol>>,
}

impl DmxOutputManager {
    pub fn init(cx: &mut AppContext) {
        cx.set_global::<Self>(Self {
            protocols: Vec::new(),
        });

        cx.spawn(|mut cx| async move {
            loop {
                cx.update_global(|this: &mut Self, cx| {
                    ShowfileManager::show(cx).recalculate_stage_output();
                    let stage_output = ShowfileManager::show(cx).stage_output().borrow().clone();

                    for protocol in this.protocols.iter_mut() {
                        if let Err(error) = protocol.send_dmx_output(&stage_output) {
                            log::error!("Failed to send DMX output: {error}");
                        }
                    }
                })
                .ok();

                cx.background_executor().timer(DMX_OUTPUT_RATE).await;
            }
        })
        .detach();
    }

    pub fn register_protocol<P: DmxProtocol + 'static>(protocol: P, cx: &mut AppContext) {
        cx.update_global::<Self, _>(|this, _cx| {
            this.protocols.push(Box::new(protocol));
        });
    }
}

impl Global for DmxOutputManager {}

pub trait DmxProtocol {
    fn send_dmx_output(&mut self, output: &DmxOutput) -> Result<()>;
}

pub mod artnet {
    use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

    use artnet_protocol::{ArtCommand, Output};
    use dmx::DmxOutput;

    use super::DmxProtocol;

    pub struct ArtnetDmxProtocol {
        socket: UdpSocket,
        target_address: SocketAddr,
        port_address: u16,
        local_universe: u16,
    }

    impl ArtnetDmxProtocol {
        const ARTNET_PORT: u16 = 6454;

        pub fn new(target_ip: &str, universe: u16, local_universe: u16) -> anyhow::Result<Self> {
            let target_address = (target_ip, Self::ARTNET_PORT)
                .to_socket_addrs()
                .expect("Could not resolve address")
                .next()
                .unwrap();

            let socket = UdpSocket::bind("0.0.0.0:0")?;
            socket.set_broadcast(true)?;

            Ok(Self {
                socket,
                target_address,
                port_address: universe,
                local_universe,
            })
        }
    }

    impl DmxProtocol for ArtnetDmxProtocol {
        fn send_dmx_output(&mut self, output: &DmxOutput) -> anyhow::Result<()> {
            let Some(universe) = output.universe(self.local_universe) else {
                log::trace!(
                    "Failed to get universe with id {} while trying to output DMX",
                    self.local_universe
                );
                return Ok(());
            };
            let data = universe.get_addresses().to_vec();

            let command = ArtCommand::Output(Output {
                data: data.into(),
                port_address: self.port_address.try_into()?,
                ..Output::default()
            });
            let bytes = command.write_to_buffer()?;

            self.socket.send_to(&bytes, self.target_address)?;
            Ok(())
        }
    }
}
