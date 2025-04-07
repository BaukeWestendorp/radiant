use std::time::Duration;

pub use data::DataPacket;

mod data;

pub(crate) const PREAMBLE_SIZE: u16 = 0x0010;
pub(crate) const POSTAMBLE_SIZE: u16 = 0x0000;
pub(crate) const VECTOR_ROOT_DATA: u32 = 0x00000004;
pub(crate) const _VECTOR_ROOT_EXTENDED: u32 = 0x00000008;
pub(crate) const PACKET_IDENTIFIER: [u8; 12] =
    [0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00];

// FIXME: These are to be used in the sync and discovery packets.
const _VECTOR_EXTENDED_SYNCHRONIZATION: u32 = 0x00000001;
const _VECTOR_EXTENDED_DISCOVERY: u32 = 0x00000002;
const _VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST: u32 = 0x00000001;
const _DISCOVERY_UNIVERSE: u32 = 64214;

const _UNIVERSE_DISCOVERY_INTERVAL: Duration = Duration::from_secs(10);
const _NETWORK_DATA_LOSS_TIMEOUT: Duration = Duration::from_millis(2500);

pub trait Pdu {
    fn to_bytes(&self) -> Vec<u8>;

    fn len(&self) -> u16;
}
