//! An sACN Receiver.
//!
//! Responsible for receiving and processing sACN packets.

use std::{
    net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use dmx::{Channel, Multiverse, Universe, UniverseId};
use socket2::{Domain, SockAddr, Socket, Type};

use crate::{
    DEFAULT_PORT, Error, MAX_UNIVERSE_SIZE,
    packet::{DataPacket, Packet},
};

const _NETWORK_DATA_LOSS_TIMEOUT: Duration = Duration::from_millis(2500);
const _UNIVERSE_DISCOVERY_INTERVAL: Duration = Duration::from_secs(10);

/// A sACN receiver.
///
/// Responsible for receiving and processing sACN packets.
pub struct Receiver {
    config: ReceiverConfig,
    inner: Arc<Inner>,
}

impl Receiver {
    /// Creates a new [Receiver].
    pub fn new(config: ReceiverConfig) -> Self {
        let domain = if config.ip.is_ipv4() { Domain::IPV4 } else { Domain::IPV6 };
        let socket = Socket::new(domain, Type::DGRAM, None).unwrap();

        Self {
            config,
            inner: Arc::new(Inner {
                socket,
                data: Mutex::new(Multiverse::new()),
                sync_state: Mutex::new(SynchronizationState::default()),
            }),
        }
    }

    /// Returns the [ReceiverConfig] for this [Receiver].
    pub fn config(&self) -> &ReceiverConfig {
        &self.config
    }

    /// Returns the data received by this [Receiver] represented as a [Multiverse].
    pub fn data(&self) -> Multiverse {
        self.inner.data.lock().unwrap().clone()
    }

    /// Returns whether this [Receiver] is currently synchronizing.
    pub fn is_synchronizing(&self) -> bool {
        *self.inner.sync_state.lock().unwrap() == SynchronizationState::Synchronized
    }

    /// Start this [Receiver].
    ///
    /// Calling this method will start the receiver in a new thread.
    pub fn start(&mut self) -> Result<(), Error> {
        let addr = SocketAddr::new(self.config.ip, self.config.port);

        thread::spawn({
            let inner = Arc::clone(&self.inner);
            move || -> Result<(), Error> {
                inner.socket.bind(&addr.into())?;
                inner.start_recv_loop()?;
                Ok(())
            }
        });

        Ok(())
    }

    /// Stop this [Receiver].
    pub fn stop(&self) -> Result<(), Error> {
        self.inner.socket.shutdown(Shutdown::Both)?;
        Ok(())
    }
}

/// Configuration for a [Receiver].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiverConfig {
    /// The IP address the receiver should bind to.
    pub ip: IpAddr,
    /// The port the receiver should bind to.
    pub port: u16,
}

impl Default for ReceiverConfig {
    fn default() -> Self {
        Self { ip: Ipv4Addr::UNSPECIFIED.into(), port: DEFAULT_PORT }
    }
}

/// Synchronization state of a [Receiver].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SynchronizationState {
    /// This receiver is not handling synchronization packets.
    #[default]
    Unsynchronized,
    /// This receiver is actively receiving synchronization packets
    /// within the minimum refresh window for DMX512-A packets.
    Synchronized,
}

struct Inner {
    socket: Socket,
    data: Mutex<Multiverse>,
    sync_state: Mutex<SynchronizationState>,
}

impl Inner {
    pub fn start_recv_loop(&self) -> Result<(), Error> {
        loop {
            match self.recv_packet_from() {
                Ok((packet, _)) => match packet {
                    Packet::Data(packet) => self.handle_data_packet(packet)?,
                    Packet::Discovery(_) => todo!(),
                    Packet::Sync(_) => todo!(),
                },
                Err(err) => {
                    eprintln!("Error receiving packet: {}", err);
                }
            }
        }
    }

    fn handle_data_packet(&self, packet: DataPacket) -> Result<(), Error> {
        if let Ok(universe_id) = UniverseId::new(packet.universe()) {
            let mut data = self.data.lock().unwrap();

            if !data.has_universe(&universe_id) {
                data.create_universe(universe_id, Universe::new());
            }

            if let Some(universe) = data.universe_mut(&universe_id) {
                for i in 0..MAX_UNIVERSE_SIZE {
                    universe.set_value(
                        &Channel::new(i + 1).unwrap(),
                        packet.data().get(i as usize).copied().unwrap_or_default().into(),
                    );
                }
            }
        }

        Ok(())
    }

    fn recv_packet_from(&self) -> Result<(Packet, SockAddr), Error> {
        const MAX_PACKET_SIZE: usize = 1144;

        let mut buffer = Vec::with_capacity(MAX_PACKET_SIZE);
        let (received, addr) = self.socket.recv_from(buffer.spare_capacity_mut())?;
        // SAFETY: just received into the `buffer`.
        unsafe {
            buffer.set_len(received);
        }

        let packet = Packet::from_bytes(&buffer)?;

        Ok((packet, addr))
    }
}
