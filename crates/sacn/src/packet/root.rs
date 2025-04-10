use super::{PacketError, Pdu, Postamble, Preamble, flags_and_length};
use crate::{
    ComponentIdentifier,
    acn::{self, Postamble as _, Preamble as _},
};

/// An E1.31 Root Layer.
pub struct RootLayer {
    cid: ComponentIdentifier,
    extended: bool,
    pdu: Pdu,
}

impl RootLayer {
    const VECTOR: [u8; 4] = [0x00, 0x00, 0x00, 0x04];
    const VECTOR_EXTENDED: [u8; 4] = [0x00, 0x00, 0x00, 0x08];

    /// Creates a new [RootLayer].
    pub fn new(cid: ComponentIdentifier, extended: bool, pdu: Pdu) -> Self {
        Self { cid, extended, pdu }
    }

    /// The CID in this layer.
    pub fn cid(&self) -> &ComponentIdentifier {
        &self.cid
    }

    /// The PDU in this layer.
    pub fn pdu(&self) -> &Pdu {
        &self.pdu
    }
}

impl acn::Pdu for RootLayer {
    type DecodeError = PacketError;

    fn encode(&self) -> impl Into<Vec<u8>> {
        // E1.31 Flags & Length
        let flags_and_length = flags_and_length(self.size()).to_be_bytes();

        // E1.31 Vector
        let vector = if self.extended { Self::VECTOR_EXTENDED } else { Self::VECTOR };

        // E1.31 CID (Component Identifier)
        let cid = self.cid.as_bytes();

        let mut bytes = Vec::with_capacity(self.size());
        bytes.extend_from_slice(&flags_and_length);
        bytes.extend_from_slice(&vector);
        bytes.extend_from_slice(cid);
        bytes.extend(self.pdu.encode().into());
        bytes
    }

    fn decode(data: &[u8]) -> Result<Self, Self::DecodeError> {
        const MIN_ROOT_LAYER_SIZE: usize = 37;

        if data.len() < MIN_ROOT_LAYER_SIZE {
            return Err(PacketError::InvalidRootLayerSize(data.len()));
        }

        // E1.31 Vector
        let vector = [data[18], data[19], data[20], data[21]];
        let extended = match vector {
            Self::VECTOR => false,
            Self::VECTOR_EXTENDED => true,
            _ => return Err(PacketError::InvalidRootLayerVector(vector.to_vec())),
        };

        // E1.31 CID (Component Identifier)
        let cid = ComponentIdentifier::from_bytes(data[22..38].try_into().unwrap());

        // E1.31 PDU
        let pdu = Pdu::decode(&data[38..])?;

        Ok(Self::new(cid, extended, pdu))
    }

    fn size(&self) -> usize {
        self.pdu.size() - Preamble::SIZE - Postamble.size()
    }
}
