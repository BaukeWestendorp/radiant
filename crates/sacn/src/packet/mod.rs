use std::time::Duration;

pub use data::DataPacket;
pub use sync::SynchronizationPacket;

use crate::ComponentIdentifier;

mod data;
mod sync;

// FIXME: These are to be used in the sync and discovery packets.
const _VECTOR_EXTENDED_DISCOVERY: u32 = 0x00000002;
const _VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST: u32 = 0x00000001;
const _DISCOVERY_UNIVERSE: u32 = 64214;

const _UNIVERSE_DISCOVERY_INTERVAL: Duration = Duration::from_secs(10);
const _NETWORK_DATA_LOSS_TIMEOUT: Duration = Duration::from_millis(2500);

pub trait Pdu {
    fn to_bytes(&self) -> Vec<u8>;

    fn len(&self) -> u16;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RootLayer {
    cid: ComponentIdentifier,
    vector: u32,
}

impl RootLayer {
    pub const PREAMBLE_SIZE: u16 = 0x0010;
    pub const POSTAMBLE_SIZE: u16 = 0x0000;
    pub const VECTOR_ROOT_DATA: u32 = 0x00000004;
    pub const VECTOR_ROOT_EXTENDED: u32 = 0x00000008;
    pub const PACKET_IDENTIFIER: [u8; 12] =
        [0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00];

    pub fn new(cid: ComponentIdentifier, extended: bool) -> Self {
        let vector = if extended { Self::VECTOR_ROOT_EXTENDED } else { Self::VECTOR_ROOT_DATA };
        RootLayer { cid, vector }
    }

    pub fn to_bytes(&self, pdu_len: u16) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(54);
        bytes.extend(Self::PREAMBLE_SIZE.to_be_bytes());
        bytes.extend(Self::POSTAMBLE_SIZE.to_be_bytes());
        bytes.extend(Self::PACKET_IDENTIFIER);
        bytes.extend(flags_and_length(pdu_len - 16).to_be_bytes());
        bytes.extend(self.vector.to_be_bytes());
        bytes.extend(self.cid.as_bytes());
        bytes
    }
}

pub(crate) fn flags_and_length(pdu_len: u16) -> u16 {
    // Low 12 bits = PDU length, high 4 bits = 0x7.
    let flags = 0x7 << 12;
    let length = pdu_len & 0xFFF;
    flags | length
}
