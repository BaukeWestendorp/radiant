use crate::{ComponentIdentifier, Error};

pub use data::DataPacket;
pub use discovery::UniverseDiscoveryPacket;
pub use sync::SynchronizationPacket;

mod data;
mod discovery;
mod sync;

pub const ROOT_PREAMBLE_SIZE: u16 = 0x0010;
pub const ROOT_POSTAMBLE_SIZE: u16 = 0x0000;
pub const ROOT_VECTOR_ROOT_DATA: u32 = 0x00000004;
pub const ROOT_VECTOR_ROOT_EXTENDED: u32 = 0x00000008;
pub const ROOT_PACKET_IDENTIFIER: [u8; 12] =
    [0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    Data(DataPacket),
    Discovery(UniverseDiscoveryPacket),
    Sync(SynchronizationPacket),
}

impl Packet {
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

pub trait Pdu {
    fn to_bytes(&self) -> Vec<u8>;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;

    fn len(&self) -> u16;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RootLayer {
    cid: ComponentIdentifier,
    vector: u32,
}

impl RootLayer {
    pub fn new(cid: ComponentIdentifier, extended: bool) -> Self {
        let vector = if extended { ROOT_VECTOR_ROOT_EXTENDED } else { ROOT_VECTOR_ROOT_DATA };
        RootLayer { cid, vector }
    }

    pub fn to_bytes(&self, pdu_len: u16) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(54);
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
