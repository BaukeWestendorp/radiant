//! An sACN Receiver.
//!
//! Responsible for receiving and processing sACN packets.

use crate::{
    DEFAULT_PORT, MAX_UNIVERSE_SIZE,
    packet::{DataFraming, Packet, Pdu, SyncFraming},
};
use socket2::{Domain, SockAddr, Socket, Type};
use std::{
    net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr},
    sync::{Arc, mpsc},
    thread,
    time::Duration,
};

const _NETWORK_DATA_LOSS_TIMEOUT: Duration = Duration::from_millis(2500);

/// Error type returned by a [Receiver].
#[derive(Debug, thiserror::Error)]
pub enum ReceiverError {
    /// An [std::io::Error] wrapper.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// A sACN receiver.
///
/// Responsible for receiving and processing sACN packets.
pub struct Receiver {
    config: ReceiverConfig,
    inner: Arc<Inner>,
    rx: mpsc::Receiver<(u16, Vec<u8>)>,
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

        let inner = Arc::new(Inner { socket });

        let (tx, rx) = mpsc::channel();
        thread::spawn({
            let inner = Arc::clone(&inner);
            move || {
                inner.start(tx).unwrap();
            }
        });

        Ok(Self { config, inner, rx })
    }

    /// Shut down this [Receiver].
    pub fn shutdown(&self) -> Result<(), ReceiverError> {
        self.inner.socket.shutdown(Shutdown::Both)?;
        Ok(())
    }

    /// Attempts to wait for a value on this receiver.
    ///
    /// This method will block the current thread until a value is received on this receiver.
    ///
    /// # Errors
    ///
    /// This function will return an error if the receiver has been shut down.
    pub fn recv(&self) -> Result<(u16, Vec<u8>), mpsc::RecvError> {
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
    pub fn recv_timeout(
        &self,
        timeout: Duration,
    ) -> Result<(u16, Vec<u8>), mpsc::RecvTimeoutError> {
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
    pub fn try_recv(&self) -> Result<(u16, Vec<u8>), mpsc::TryRecvError> {
        self.rx.try_recv()
    }

    /// Returns the [ReceiverConfig] for this [Receiver].
    pub fn config(&self) -> &ReceiverConfig {
        &self.config
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
    socket: Socket,
}

impl Inner {
    pub fn start(&self, tx: mpsc::Sender<(u16, Vec<u8>)>) -> Result<(), ReceiverError> {
        loop {
            match self.recv_packet_from()? {
                Some((packet, _)) => match &packet.block.pdus()[0].pdu() {
                    Pdu::DataFraming(pdu) => {
                        let (id, universe) = self.universe_from_data_framing(pdu)?;
                        tx.send((id, universe)).expect("channel should not be closed");
                    }
                    Pdu::SyncFraming(sync_framing) => self.handle_sync_framing(sync_framing),
                    Pdu::DiscoveryFraming(_) => self.handle_discovery_framing(),
                },
                None => {}
            }
        }
    }

    fn recv_packet_from(&self) -> Result<Option<(Packet, SockAddr)>, ReceiverError> {
        const MAX_PACKET_SIZE: usize = 1144;

        let mut data = Vec::with_capacity(MAX_PACKET_SIZE);
        let (received, addr) = self.socket.recv_from(data.spare_capacity_mut())?;
        // SAFETY: just received into the `buffer`.
        unsafe {
            data.set_len(received);
        }

        match Packet::decode(&data) {
            Ok(packet) => Ok(Some((packet, addr))),
            Err(err) => {
                eprintln!("Invalid packet discarded: {err:?}");
                Ok(None)
            }
        }
    }

    fn universe_from_data_framing(
        &self,
        data_framing: &DataFraming,
    ) -> Result<(u16, Vec<u8>), ReceiverError> {
        let universe_id = data_framing.universe();
        let data = data_framing.dmp().data();

        let mut universe = Vec::new();
        for i in 0..MAX_UNIVERSE_SIZE {
            let value = data.get(i as usize).copied().unwrap_or_default().into();
            universe.push(value);
        }

        Ok((universe_id, universe))
    }

    fn handle_sync_framing(&self, _sync_framing: &SyncFraming) {
        // Handle sync framing logic here
    }

    fn handle_discovery_framing(&self) {
        // Handle discovery framing logic here
    }
}
