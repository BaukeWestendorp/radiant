use std::time::Duration;

pub use data::DataPacket;

mod data;

pub(crate) const PREAMBLE_SIZE: u16 = 0x0010;
pub(crate) const POSTAMBLE_SIZE: u16 = 0x0000;
pub(crate) const VECTOR_ROOT_DATA: u32 = 0x00000004;
pub(crate) const VECTOR_ROOT_EXTENDED: u32 = 0x00000008;
pub(crate) const VECTOR_DMP_SET_PROPERTY: u8 = 0x02;
pub(crate) const VECTOR_DATA_PACKET: u32 = 0x00000002;
pub(crate) const VECTOR_EXTENDED_SYNCHRONIZATION: u32 = 0x00000001;
pub(crate) const VECTOR_EXTENDED_DISCOVERY: u32 = 0x00000002;
pub(crate) const VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST: u32 = 0x00000001;
pub(crate) const UNIVERSE_DISCOVERY_INTERVAL: Duration = Duration::from_secs(10);
pub(crate) const NETWORK_DATA_LOSS_TIMEOUT: Duration = Duration::from_millis(2500);
pub(crate) const DISCOVERY_UNIVERSE: u32 = 64214;
pub(crate) const ACN_SDT_MULTICAST_PORT: u16 = 5568;

pub(crate) const ACN_PACKET_IDENTIFIER: [u8; 12] =
    [0x41, 0x53, 0x43, 0x4e, 0x2d, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00];

pub trait Pdu {
    fn to_bytes(&self) -> Vec<u8>;

    fn from_bytes(_bytes: &[u8]) -> Self
    // FIXME: Remove bound
    where
        Self: Sized,
    {
        todo!("implement parsing PDUs");
    }

    fn len(&self) -> u16;
}
