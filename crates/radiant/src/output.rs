use anyhow::Result;
use backstage::{
    dmx::DmxOutput,
    show::graph::{GraphState, NodeKind},
};
use gpui::{AppContext, BorrowAppContext, Global};
use std::time::Duration;

use crate::showfile::Showfile;

pub const DMX_UPDATE_RATE: Duration = Duration::from_millis(1000 / 40);

#[derive(Default)]
pub struct OutputManager {
    protocols: Vec<Box<dyn DmxProtocol>>,
}

impl OutputManager {
    pub fn init(cx: &mut AppContext) {
        cx.spawn(|mut cx| async move {
            loop {
                cx.update_global::<Showfile, _>(|showfile, cx| {
                    let dmx_output = showfile.show.get_dmx_output();

                    cx.update_global::<Self, _>(|output_manager, _cx| {
                        for protocol in output_manager.protocols.iter_mut() {
                            if let Err(error) = protocol.send_dmx_output(&dmx_output) {
                                log::error!("Failed to send DMX output: {error}");
                            }
                        }
                    });
                })
                .map_err(|err| log::error!("Failed to send DMX output: {err}"))
                .ok();

                cx.background_executor().timer(DMX_UPDATE_RATE).await;
            }
        })
        .detach();

        cx.set_global(Self::default());
    }

    pub fn register_protocol<P: DmxProtocol + 'static>(protocol: P, cx: &mut AppContext) {
        cx.update_global::<Self, _>(|this, _cx| {
            this.protocols.push(Box::new(protocol));
        });
    }
}

impl Global for OutputManager {}

pub trait DmxProtocol {
    fn send_dmx_output(&mut self, output: &DmxOutput) -> Result<()>;
}

pub mod artnet {
    use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

    use anyhow::anyhow;
    use artnet_protocol::{ArtCommand, Output};
    use backstage::dmx::DmxOutput;

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
            let Some(target_address) = (target_ip, Self::ARTNET_PORT).to_socket_addrs()?.next()
            else {
                return Err(anyhow!("Failed to parse target address:"));
            };

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
            let Some(universe) = output.get_universe(self.local_universe) else {
                log::debug!(
                    "Failed to get universe with id {} while trying to output DMX",
                    self.local_universe
                );
                return Ok(());
            };
            let data = universe.get_channels().to_vec();

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
