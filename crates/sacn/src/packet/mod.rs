//! # Packets
//!
//! sACN has three packet types:
//! - Data Packets
//! - Universe Discovery Packets
//! - Synchronization Packets

use crate::ComponentIdentifier;
use crate::acn::{self, Pdu as _, PduBlock};
use crate::packet::data::DataFraming;
use discovery::DiscoveryFraming;
use root::RootLayer;
use sync::SyncFraming;

pub mod data;
pub mod discovery;
pub mod root;
pub mod sync;

pub struct Packet(acn::Packet<Preamble, RootLayer, Postamble>);

impl Packet {
    pub fn new(cid: ComponentIdentifier, pdu: Pdu) -> Self {
        let extended = match pdu {
            Pdu::DataFraming(_) => false,
            Pdu::SyncFraming(_) | Pdu::DiscoveryFraming(_) => true,
        };

        let root_layer_pdu = RootLayer::new(cid, extended, pdu);
        let packet = acn::Packet::new(Preamble, PduBlock::new(vec![root_layer_pdu]), Postamble);

        Self(packet)
    }

    pub fn decode(data: &[u8]) -> Result<Self, crate::Error> {
        let root_layer = RootLayer::decode(data)?;
        Ok(Self(acn::Packet::new(Preamble, PduBlock::new(vec![root_layer]), Postamble)))
    }

    pub fn encode(&self) -> impl Into<Vec<u8>> {
        self.0.encode()
    }
}

impl std::ops::Deref for Packet {
    type Target = acn::Packet<Preamble, RootLayer, Postamble>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Preamble;

impl Preamble {
    #[rustfmt::skip]
    pub const BYTES: [u8; 16 as usize] = {
        [
            0x00, 0x10, // E1.31 RLP Preamble Size
            0x00, 0x00, // E1.31 RLP Postamble Size
            0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00, // E1.31 ACN Packet Identifier
        ]
    };
}

impl acn::Preamble for Preamble {
    type DecodeError = crate::Error;

    const SIZE: usize = Self::BYTES.len();

    fn encode(&self) -> impl Into<Vec<u8>> {
        Self::BYTES
    }

    fn decode(data: &[u8]) -> Result<Self, Self::DecodeError> {
        // E1.31 5.1 Preamble Size
        if data[0..2] != Self::BYTES[0..2] {
            return Err(crate::Error::InvalidPreamblePreambleSize(u16::from_be_bytes([
                data[0], data[1],
            ])));
        }

        // E1.31 5.2 Postamble Size
        if data[2..4] != Self::BYTES[2..4] {
            return Err(crate::Error::InvalidPreamblePostambleSize(u16::from_be_bytes([
                data[2], data[3],
            ])));
        }

        // E1.31 5.3 ACN Packet Identifier
        if data[4..16] != Self::BYTES[4..16] {
            return Err(crate::Error::InvalidPreambleAcnPacketIdentifier(data[4..16].to_vec()));
        }

        Ok(Self)
    }
}

pub struct Postamble;

impl acn::Postamble for Postamble {
    type DecodeError = crate::Error;

    fn encode(&self) -> impl Into<Vec<u8>> {
        vec![]
    }

    fn decode(_data: &[u8]) -> Result<Self, Self::DecodeError> {
        Ok(Self)
    }

    fn size(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pdu {
    DataFraming(DataFraming),
    SyncFraming(SyncFraming),
    DiscoveryFraming(DiscoveryFraming),
}

impl acn::Pdu for Pdu {
    type DecodeError = crate::Error;

    fn encode(&self) -> impl Into<Vec<u8>> {
        match self {
            Pdu::DataFraming(pdu) => pdu.encode().into(),
            Pdu::SyncFraming(pdu) => pdu.encode().into(),
            Pdu::DiscoveryFraming(pdu) => pdu.encode().into(),
        }
    }

    fn decode(data: &[u8]) -> Result<Self, Self::DecodeError> {
        if let Ok(data_framing) = DataFraming::decode(data) {
            return Ok(Pdu::DataFraming(data_framing));
        }

        if let Ok(sync_framing) = SyncFraming::decode(data) {
            return Ok(Pdu::SyncFraming(sync_framing));
        }

        if let Ok(discovery_framing) = DiscoveryFraming::decode(data) {
            return Ok(Pdu::DiscoveryFraming(discovery_framing));
        }

        Err(crate::Error::InvalidPacket)
    }

    fn size(&self) -> usize {
        match self {
            Pdu::DataFraming(pdu) => pdu.size(),
            Pdu::SyncFraming(pdu) => pdu.size(),
            Pdu::DiscoveryFraming(pdu) => pdu.size(),
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
