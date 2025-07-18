//! An sACN Receiver.
//!
//! Responsible for receiving and processing sACN packets.

use super::{
    DEFAULT_PORT, Universe,
    packet::{DataFraming, DiscoveryFraming, Packet, PacketError, Pdu, SyncFraming},
};
use socket2::{Domain, Socket, Type};
use std::{
    net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr},
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
    time::Duration,
};

const _NETWORK_DATA_LOSS_TIMEOUT: Duration = Duration::from_millis(2500);

/// Error type returned by a [Receiver].
#[derive(Debug, thiserror::Error)]
pub enum ReceiverError {
    /// An [std::io::Error] wrapper.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// A [PacketError] wrapper.
    #[error(transparent)]
    InvalidPacket(#[from] PacketError),

    /// The connection was closed.
    #[error("Connection closed")]
    NoData,
}

/// A sACN receiver.
///
/// Responsible for receiving and processing sACN packets.
pub struct Receiver {
    inner: Arc<Inner>,
    rx: mpsc::Receiver<Universe>,
    thread_handle: Option<JoinHandle<()>>,
}

impl Receiver {
    /// Creates a new [Receiver].
    pub fn start(config: ReceiverConfig) -> Result<Self, ReceiverError> {
        let domain = if config.ip.is_ipv4() { Domain::IPV4 } else { Domain::IPV6 };
        let addr = SocketAddr::new(config.ip, config.port);
        let socket: Socket = Socket::new(domain, Type::DGRAM, None)?;
        socket.set_reuse_address(true)?;
        socket.set_reuse_port(true)?;
        socket.bind(&addr.into())?;

        log::info!("bound sACN Receiver on {}:{}", addr, config.port);

        let inner = Arc::new(Inner { config: Mutex::new(config), socket });

        let (tx, rx) = mpsc::channel();
        let thread_handle = thread::spawn({
            let inner = Arc::clone(&inner);
            move || {
                inner.start(&tx).unwrap();
            }
        });

        Ok(Self { thread_handle: Some(thread_handle), inner, rx })
    }

    /// Shut down this [Receiver].
    pub fn shutdown(&mut self) -> Result<(), ReceiverError> {
        log::info!("shutting down sACN Receiver");
        self.inner.socket.shutdown(Shutdown::Both)?;
        self.thread_handle.take().unwrap().join().ok();
        Ok(())
    }

    /// Attempts to wait for a value on this receiver.
    ///
    /// This method will block the current thread until a value is received on this receiver.
    ///
    /// # Errors
    ///
    /// This function will return an error if the receiver has been shut down.
    pub fn recv(&self) -> Result<Universe, mpsc::RecvError> {
        self.rx.recv()
    }

    /// Attempts to wait for a value on this receiver, returning an error if
    /// the corresponding channel has hung up, or if it waits more than timeout.
    /// This function will always block the current thread if there is no data
    /// available and it’s possible for more data to be sent (the receiver is not shut down).
    ///
    /// # Errors
    ///
    /// This function will return an error if the receiver has been shut down or the timeout is reached.
    pub fn recv_timeout(&self, timeout: Duration) -> Result<Universe, mpsc::RecvTimeoutError> {
        self.rx.recv_timeout(timeout)
    }

    /// Attempts to return a pending value on this receiver without blocking.
    /// This method will never block the caller in order to wait for data to become available.
    /// Instead, this will always return immediately with a possible option of pending data on the channel.
    /// This is useful for a flavor of “optimistic check” before deciding to block on a receiver.
    ///
    /// Compared with `recv`, this function has two failure cases instead of one
    /// (one for disconnection, one for an empty buffer).
    ///
    /// # Errors
    ///
    /// This function will return an error if the receiver has been shut down or
    pub fn try_recv(&self) -> Result<Universe, mpsc::TryRecvError> {
        self.rx.try_recv()
    }

    /// Returns the [ReceiverConfig] for this [Receiver].
    pub fn config(&self) -> ReceiverConfig {
        self.inner.config.lock().unwrap().clone()
    }

    /// Sets the configuration for this [Source].
    pub fn set_config(&self, config: ReceiverConfig) {
        *self.inner.config.lock().unwrap() = config;
    }
}

impl Drop for Receiver {
    fn drop(&mut self) {
        self.shutdown().ok();
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

struct Inner {
    config: Mutex<ReceiverConfig>,
    socket: Socket,
}

impl Inner {
    pub fn start(&self, tx: &mpsc::Sender<Universe>) -> Result<(), ReceiverError> {
        log::debug!("starting sACN Receiver");
        loop {
            let packet = match self.recv_packet() {
                Ok(packet) => {
                    log::debug!("received packet: {packet:?}");
                    packet
                }
                Err(ReceiverError::InvalidPacket(packet_err)) => {
                    log::warn!("received invalid packet: {packet_err}");
                    continue;
                }
                Err(ReceiverError::NoData) => {
                    return Ok(());
                }
                Err(err) => return Err(err),
            };

            let root =
                &packet.block.pdus().first().expect("sACN packet should contain at least one PDU");

            match &root.pdu() {
                Pdu::DataFraming(pdu) => {
                    let universe = self.universe_from_data_framing(pdu)?;
                    tx.send(universe).expect("channel should not be closed");
                }
                Pdu::SyncFraming(sync_framing) => self.handle_sync_framing(sync_framing),
                Pdu::DiscoveryFraming(discovery_framing) => {
                    self.handle_discovery_framing(discovery_framing)
                }
            }
        }
    }

    fn recv_packet(&self) -> Result<Packet, ReceiverError> {
        const MAX_PACKET_SIZE: usize = 1144;

        let mut data = Vec::with_capacity(MAX_PACKET_SIZE);
        let buffer = data.spare_capacity_mut();
        let received = self.socket.recv(buffer)?;

        if received == 0 {
            return Err(ReceiverError::NoData);
        }

        // SAFETY: just received into the `buffer`.
        unsafe {
            data.set_len(received);
        }

        Ok(Packet::decode(&data)?)
    }

    fn universe_from_data_framing(
        &self,
        data_framing: &DataFraming,
    ) -> Result<Universe, ReceiverError> {
        let universe_number = data_framing.universe();
        let start_code_slot = data_framing.dmp().start_code_slot();
        let data_slots = data_framing.dmp().data_slots();

        let mut universe = Universe::new(universe_number);
        universe.start_code_slot = start_code_slot;
        universe.data_slots.extend(data_slots.to_owned());

        Ok(universe)
    }

    fn handle_sync_framing(&self, _sync_framing: &SyncFraming) {
        // Handle sync framing logic here
    }

    fn handle_discovery_framing(&self, _discovery_framing: &DiscoveryFraming) {
        // Handle discovery framing logic here
    }
}
