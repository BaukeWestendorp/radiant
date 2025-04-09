//! # Packets
//!
//! sACN has three packet types:
//! - Data Packets
//! - Universe Discovery Packets
//! - Synchronization Packets

use crate::{ComponentIdentifier, Error};

pub use data::DataPacket;
pub use discovery::UniverseDiscoveryPacket;
pub use sync::SynchronizationPacket;

mod data;
mod discovery;
mod sync;

const ROOT_PREAMBLE_SIZE: u16 = 0x0010;
const ROOT_POSTAMBLE_SIZE: u16 = 0x0000;
const ROOT_VECTOR_ROOT_DATA: u32 = 0x00000004;
const ROOT_VECTOR_ROOT_EXTENDED: u32 = 0x00000008;
const ROOT_PACKET_IDENTIFIER: [u8; 12] =
    [0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00];

/// An E1.31 Packet
///
/// Any of the set of packets containing E1.31 Data Packets, E1.31 Synchronization Packets, and E1.31 Universe Discovery Packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    /// E1.31 Data Packet
    Data(DataPacket),
    /// E1.31 Universe Discovery Packet
    Discovery(UniverseDiscoveryPacket),
    /// E1.31 Synchronization Packet
    Sync(SynchronizationPacket),
}

impl Packet {
    /// Converts the packet into a network ordered byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Packet::Data(packet) => packet.to_bytes(),
            Packet::Discovery(packet) => packet.to_bytes(),
            Packet::Sync(packet) => packet.to_bytes(),
        }
    }

    /// Converts a network ordered byte vector into a packet.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if let Ok(data_packet) = DataPacket::from_bytes(bytes) {
            return Ok(Packet::Data(data_packet));
        }
        if let Ok(discovery_packet) = UniverseDiscoveryPacket::from_bytes(bytes) {
            return Ok(Packet::Discovery(discovery_packet));
        }
        if let Ok(sync_packet) = SynchronizationPacket::from_bytes(bytes) {
            return Ok(Packet::Sync(sync_packet));
        }
        Err(Error::InvalidPacket)
    }
}

/// [Pdu] (Protocol Data Unit) is a wrapper for packets in the ANSI E1.17.
///
/// This crate uses it to provide a common interface for working with every sACN packet.
pub trait Pdu {
    /// Converts the packet into a network ordered byte vector.
    fn to_bytes(&self) -> Vec<u8>;

    /// Converts a network ordered byte vector into a packet.
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;

    /// Returns the length of the PDU in bytes.
    fn len(&self) -> u16;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RootLayer {
    cid: ComponentIdentifier,
    vector: u32,
}

impl RootLayer {
    pub const SIZE: usize = 38;

    pub fn new(cid: ComponentIdentifier, extended: bool) -> Self {
        let vector = if extended { ROOT_VECTOR_ROOT_EXTENDED } else { ROOT_VECTOR_ROOT_DATA };
        RootLayer { cid, vector }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        // E1.31 5.1 Preamble Size
        let preamble_size = u16::from_be_bytes([bytes[0], bytes[1]]);
        if preamble_size != ROOT_PREAMBLE_SIZE {
            return Err(Error::InvalidPreambleSize(preamble_size));
        }

        // E1.31 5.2 Post-amble Size
        let post_amble_size = u16::from_be_bytes([bytes[2], bytes[3]]);
        if post_amble_size != ROOT_POSTAMBLE_SIZE {
            return Err(Error::InvalidPostambleSize(post_amble_size));
        }

        // E1.31 5.3 ACN Packet Identifier
        let packet_identifier = &bytes[4..16];
        if packet_identifier != ROOT_PACKET_IDENTIFIER {
            return Err(Error::InvalidAcnPacketIdentifier);
        }

        // E1.31 5.5 Vector
        let vector = &bytes[18..22];
        let vector = u32::from_be_bytes([vector[0], vector[1], vector[2], vector[3]]);
        if vector != ROOT_VECTOR_ROOT_EXTENDED || vector != ROOT_VECTOR_ROOT_DATA {
            return Err(Error::InvalidExtendedRootVector(vector));
        }

        // E1.31 5.6 CID
        let cid = ComponentIdentifier::from_slice(&bytes[22..38])
            .map_err(|_| Error::InvalidComponentId)?;

        Ok(RootLayer { cid, vector })
    }

    pub fn to_bytes(&self, pdu_len: u16) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::SIZE);
        bytes.extend(ROOT_PREAMBLE_SIZE.to_be_bytes());
        bytes.extend(ROOT_POSTAMBLE_SIZE.to_be_bytes());
        bytes.extend(ROOT_PACKET_IDENTIFIER);
        bytes.extend(flags_and_length(pdu_len - 16).to_be_bytes());
        bytes.extend(self.vector.to_be_bytes());
        bytes.extend(self.cid.as_bytes());
        bytes
    }
}

pub(crate) fn source_name_from_str(source_name: &str) -> Result<[u8; 64], Error> {
    // 6.2.2 E1.31 Data Packet: Source Name.
    if source_name.len() > 64 {
        return Err(Error::InvalidSourceNameLength(source_name.len()));
    }

    let bytes = source_name.as_bytes();
    let mut source_name = [0u8; 64];
    let len = bytes.len().min(64);
    source_name[..len].copy_from_slice(&bytes[..len]);
    Ok(source_name)
}

pub(crate) fn flags_and_length(pdu_len: u16) -> u16 {
    // Low 12 bits = PDU length, high 4 bits = 0x7.
    let flags = 0x7 << 12;
    let length = pdu_len & 0xFFF;
    flags | length
}
