use super::{RootLayer, flags_and_length};
use crate::{ComponentIdentifier, Error, source::SourceConfig};

pub const VECTOR_EXTENDED_SYNCHRONIZATION: u32 = 0x00000001;

pub struct SynchronizationPacket {
    root: RootLayer,
    framing: FramingLayer,
}

impl SynchronizationPacket {
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

    pub fn from_source_config(
        config: &SourceConfig,
        sequence_number: u8,
        synchronization_address: u16,
    ) -> Result<Self, Error> {
        Self::new(config.cid, sequence_number, synchronization_address)
    }
}

impl super::Pdu for SynchronizationPacket {
    fn to_bytes(&self) -> Vec<u8> {
        let pdu_len = self.len();

        vec![self.root.to_bytes(pdu_len), self.framing.to_bytes(pdu_len)].concat()
    }

    fn len(&self) -> u16 {
        49
    }
}

struct FramingLayer {
    sequence_number: u8,
    synchronization_address: u16,
}

impl FramingLayer {
    pub fn new(sequence_number: u8, synchronization_address: u16) -> Self {
        Self { sequence_number, synchronization_address }
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
