use crate::{Error, acn};

use super::flags_and_length;

/// An E1.31 Synchronization Framing Layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncFraming {
    sequence_number: u8,
    synchronization_address: u16,
}

impl SyncFraming {
    const VECTOR: [u8; 4] = [0x00, 0x00, 0x00, 0x01];

    /// Creates a new [SyncFraming] layer.
    pub fn new(sequence_number: u8, synchronization_address: u16) -> Self {
        Self { sequence_number, synchronization_address }
    }

    /// The sequence number in this layer.
    pub fn sequence_number(&self) -> u8 {
        self.sequence_number
    }

    /// The synchronization address in this layer.
    pub fn synchronization_address(&self) -> u16 {
        self.synchronization_address
    }
}

impl acn::Pdu for SyncFraming {
    type DecodeError = Error;

    fn decode(bytes: &[u8]) -> Result<Self, Self::DecodeError> {
        // E1.31 6.3.1 Synchronization Packet: Vector
        let vector = [bytes[40], bytes[41], bytes[42], bytes[43]];
        if vector != Self::VECTOR {
            return Err(Error::InvalidFramingLayerVector(vector.to_vec()));
        }

        // E1.31 6.3.2 Synchronization Packet: Sequence Number
        let sequence_number = bytes[44];

        // E1.31 6.3.3 Synchronization Packet: Synchronization Address
        let synchronization_address = u16::from_be_bytes([bytes[45], bytes[46]]);

        Ok(Self { sequence_number, synchronization_address })
    }

    fn encode(&self) -> impl Into<Vec<u8>> {
        let flags_and_length = flags_and_length(self.size()).to_be_bytes();

        let mut bytes = Vec::with_capacity(self.size());
        bytes.extend(flags_and_length);
        bytes.extend(Self::VECTOR);
        bytes.push(self.sequence_number);
        bytes.extend(self.synchronization_address.to_be_bytes());
        bytes.extend([0x00, 0x00]);
        bytes
    }

    fn size(&self) -> usize {
        11
    }
}
