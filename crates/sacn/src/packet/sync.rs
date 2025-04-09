use super::{RootLayer, flags_and_length};
use crate::{ComponentIdentifier, Error, source::SourceConfig};

const VECTOR_EXTENDED_SYNCHRONIZATION: u32 = 0x00000001;

/// Represents an E1.31 Synchronization Packet.
///
/// This packed contains only universe synchronization information and no additional data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynchronizationPacket {
    root: RootLayer,
    framing: FramingLayer,
}

impl SynchronizationPacket {
    /// Creates a new [SynchronizationPacket].
    pub fn new(
        cid: ComponentIdentifier,
        sequence_number: u8,
        synchronization_address: u16,
    ) -> Result<Self, Error> {
        Ok(Self {
            root: RootLayer::new(cid, true),
            framing: FramingLayer::new(sequence_number, synchronization_address),
        })
    }

    /// Creates a new [SynchronizationPacket] from a [SourceConfig].
    pub fn from_source_config(
        config: &SourceConfig,
        sequence_number: u8,
        synchronization_address: u16,
    ) -> Result<Self, Error> {
        Self::new(config.cid, sequence_number, synchronization_address)
    }

    /// The [ComponentIdentifier] in this packet.
    pub fn cid(&self) -> &ComponentIdentifier {
        &self.root.cid
    }

    /// The sequence number in this packet.
    pub fn sequence_number(&self) -> u8 {
        self.framing.sequence_number
    }

    /// The synchronization address in this packet.
    pub fn synchronization_address(&self) -> u16 {
        self.framing.synchronization_address
    }
}

impl super::Pdu for SynchronizationPacket {
    fn to_bytes(&self) -> Vec<u8> {
        let pdu_len = self.len();
        vec![self.root.to_bytes(pdu_len), self.framing.to_bytes(pdu_len)].concat()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self { root: RootLayer::from_bytes(bytes)?, framing: FramingLayer::from_bytes(bytes)? })
    }

    fn len(&self) -> u16 {
        49
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FramingLayer {
    sequence_number: u8,
    synchronization_address: u16,
}

impl FramingLayer {
    pub fn new(sequence_number: u8, synchronization_address: u16) -> Self {
        Self { sequence_number, synchronization_address }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        // E1.31 6.3.1 Synchronization Packet: Vector
        let vector = u32::from_be_bytes([bytes[40], bytes[41], bytes[42], bytes[43]]);
        if vector != VECTOR_EXTENDED_SYNCHRONIZATION {
            return Err(Error::InvalidFramingVector(vector));
        }

        // E1.31 6.3.2 Synchronization Packet: Sequence Number
        let sequence_number = bytes[44];

        // E1.31 6.3.3 Synchronization Packet: Synchronization Address
        let synchronization_address = u16::from_be_bytes([bytes[45], bytes[46]]);

        Ok(Self { sequence_number, synchronization_address })
    }

    pub fn to_bytes(&self, pdu_len: u16) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(11);
        bytes.extend(flags_and_length(pdu_len - 38).to_be_bytes());
        bytes.extend(VECTOR_EXTENDED_SYNCHRONIZATION.to_be_bytes());
        bytes.push(self.sequence_number);
        bytes.extend(self.synchronization_address.to_be_bytes());
        bytes.extend([0x00, 0x00]);
        bytes
    }
}
