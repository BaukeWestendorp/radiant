//! # Packets
//!
//! sACN has three packet types:
//! - Data Packets
//! - Universe Discovery Packets
//! - Synchronization Packets

use crate::acn::{self, Postamble as _, Preamble as _};

pub use data::DataPacket;
pub use discovery::UniverseDiscoveryPacket;
pub use sync::SynchronizationPacket;

mod data;
mod discovery;
mod root;
mod sync;

pub struct Packet {
    packet: acn::Packet<Preamble, Pdu, Postamble>,
}

impl Packet {
    pub fn new(packet: acn::Packet<Preamble, Pdu, Postamble>) -> Self {
        Self { packet }
    }
}

impl std::ops::Deref for Packet {
    type Target = acn::Packet<Preamble, Pdu, Postamble>;

    fn deref(&self) -> &Self::Target {
        &self.packet
    }
}

pub struct Preamble([u8; Preamble::SIZE]);

impl Preamble {
    #[rustfmt::skip]
    const BYTES: [u8; 16 as usize] = {
        [
            0x00, 0x10, // E1.31 RLP Preamble Size
            0x00, 0x00, // E1.31 RLP Postamble Size
            0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00, // E1.31 ACN Packet Identifier
        ]
    };
}

impl acn::Preamble for Preamble {
    type Error = crate::Error;

    const SIZE: usize = Self::BYTES.len();

    fn encode(&self) -> impl Into<Vec<u8>> {
        Self::BYTES
    }

    fn decode(data: &[u8]) -> Result<Self, acn::DecodeError> {
        // E1.31 5.1 Preamble Size
        // E1.31 5.2 Postamble Size
        // E1.31 5.3 ACN Packet Identifier
        if data[0..Self::SIZE] != Self::BYTES {
            return Err(acn::DecodeError::InvalidPreamble);
        }

        Ok(Self(data[0..16].try_into().unwrap()))
    }
}

pub struct Postamble;

impl acn::Postamble for Postamble {
    fn encode(&self) -> impl Into<Vec<u8>> {
        vec![]
    }

    fn decode(_data: &[u8]) -> Result<Self, acn::DecodeError> {
        Ok(Self)
    }

    fn size(&self) -> usize {
        0
    }
}

pub enum Pdu {
    DataFraming(DataFraming),
    Dmp(Dmp),
    SyncFraming(SyncFraming),
    DiscoveryFraming(DiscoveryFraming),
    UniverseDiscovery(UniverseDiscovery),
}

impl acn::Pdu for Pdu {
    fn encode(&self) -> impl Into<Vec<u8>> {
        match self {
            Self::DataFraming(pdu)
            | Self::Dmp(pdu)
            | Self::SyncFraming(pdu)
            | Self::DiscoveryFraming(pdu)
            | Self::UniverseDiscovery(pdu) => pdu.encode(),
        }
    }

    fn decode(data: &[u8]) -> Result<Self, acn::DecodeError> {
        let result = DataFraming::decode(data)
            .map(Pdu::DataFraming)
            .or_else(|_| Dmp::decode(data).map(Pdu::Dmp))
            .or_else(|_| SyncFraming::decode(data).map(Pdu::SyncFraming))
            .or_else(|_| DiscoveryFraming::decode(data).map(Pdu::DiscoveryFraming))
            .or_else(|_| UniverseDiscovery::decode(data).map(Pdu::UniverseDiscovery));

        match result {
            Ok(pdu) => Ok(pdu),
            Err(_) => Err(acn::DecodeError::InvalidPdu),
        }
    }

    fn size(&self) -> usize {
        match self {
            Self::DataFraming(pdu)
            | Self::Dmp(pdu)
            | Self::SyncFraming(pdu)
            | Self::DiscoveryFraming(pdu)
            | Self::UniverseDiscovery(pdu) => pdu.size(),
        }
    }
}

pub(crate) fn source_name_from_str(source_name: &str) -> Result<[u8; 64], crate::Error> {
    if source_name.len() > 64 {
        return Err(crate::Error::InvalidSourceNameLength(source_name.len()));
    }

    let bytes = source_name.as_bytes();
    let mut source_name = [0u8; 64];
    let len = bytes.len().min(64);
    source_name[..len].copy_from_slice(&bytes[..len]);
    Ok(source_name)
}

pub(crate) fn flags_and_length(length: usize) -> u16 {
    // Low 12 bits = PDU length, high 4 bits = 0x7.
    let flags = 0x7 << 12;
    let length = length & 0xFFF;
    flags | length as u16
}
