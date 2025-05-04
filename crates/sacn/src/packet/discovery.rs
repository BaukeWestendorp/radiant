use super::{PacketError, flags_and_length, source_name_from_str};
use crate::{acn, source::SourceConfig};

/// An E1.31 Universe Discovery Packet Framing Layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveryFraming {
    source_name: [u8; 64],
    universe_discovery: UniverseDiscovery,
}

impl DiscoveryFraming {
    const VECTOR: [u8; 4] = [0x00, 0x00, 0x00, 0x02];

    /// Creates a new [DiscoveryFraming] layer.
    pub fn new(
        source_name: &str,
        universe_discovery: UniverseDiscovery,
    ) -> Result<Self, PacketError> {
        let source_name = source_name_from_str(source_name)?;

        Ok(Self { source_name, universe_discovery })
    }

    /// The source name in this packet.
    pub fn source_name(&self) -> &str {
        core::str::from_utf8(&self.source_name).unwrap()
    }

    pub(crate) fn from_source_config(
        config: &SourceConfig,
        universe_discovery: UniverseDiscovery,
    ) -> Result<Self, PacketError> {
        Self::new(&config.name, universe_discovery)
    }
}

impl acn::Pdu for DiscoveryFraming {
    type DecodeError = PacketError;

    fn encode(&self) -> impl Into<Vec<u8>> {
        let flags_and_length = flags_and_length(self.size()).to_be_bytes();

        let mut bytes = Vec::with_capacity(self.size());
        bytes.extend(flags_and_length);
        bytes.extend(Self::VECTOR);
        bytes.extend(self.source_name);
        bytes.extend([0x00, 0x00, 0x00, 0x00]);
        bytes.extend(self.universe_discovery.encode().into());
        bytes
    }

    fn decode(bytes: &[u8]) -> Result<Self, Self::DecodeError> {
        // E1.31 6.4.1 Universe Discovery Packet: Vector
        let vector = [bytes[2], bytes[3], bytes[4], bytes[5]];
        if vector != Self::VECTOR {
            return Err(PacketError::InvalidUniverseDiscoveryLayerVector(vector.to_vec()));
        }

        // E1.31 6.4.2 Universe Discovery Packet: Source Name
        let source_name = bytes[6..70].try_into().unwrap();

        let universe_discovery = UniverseDiscovery::decode(&bytes[74..])?;

        Ok(Self { source_name, universe_discovery })
    }

    fn size(&self) -> usize {
        74 + self.universe_discovery.size()
    }
}

/// An E1.31 Universe Discovery Layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniverseDiscovery {
    /// Packet Number.
    page: u8,
    /// Final Page.
    last: u8,
    /// Sorted list of up to 512 16-bit universes.
    list_of_universes: Vec<u16>,
}

impl UniverseDiscovery {
    const VECTOR: [u8; 4] = [0x00, 0x00, 0x00, 0x01];

    /// Creates a new [UniverseDiscovery] layer.
    pub fn new(page: u8, last: u8, mut list_of_universes: Vec<u16>) -> Self {
        list_of_universes.truncate(512);
        list_of_universes.sort();
        Self { page, last, list_of_universes }
    }

    /// The page number in this layer.
    pub fn page(&self) -> u8 {
        self.page
    }

    /// The last page number in this layer.
    pub fn last(&self) -> u8 {
        self.last
    }

    /// The list of universes in this layer.
    pub fn list_of_universes(&self) -> &[u16] {
        &self.list_of_universes
    }
}

impl acn::Pdu for UniverseDiscovery {
    type DecodeError = PacketError;

    fn encode(&self) -> impl Into<Vec<u8>> {
        let flags_and_length = flags_and_length(self.size()).to_be_bytes();

        let mut bytes = Vec::with_capacity(self.size());
        bytes.extend(flags_and_length);
        bytes.extend(Self::VECTOR);
        bytes.push(self.page);
        bytes.push(self.last);
        bytes.extend(self.list_of_universes.iter().flat_map(|u| u.to_be_bytes()));
        bytes
    }

    fn decode(bytes: &[u8]) -> Result<Self, Self::DecodeError> {
        // E1.31 8.2 Universe Discovery Layer: Vector.
        let vector = [bytes[2], bytes[3], bytes[4], bytes[5]];
        if vector != Self::VECTOR {
            return Err(PacketError::InvalidUniverseDiscoveryLayerVector(vector.to_vec()));
        }

        let page = bytes[6];
        let last = bytes[7];
        let list_of_universes = bytes[8..]
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes(chunk.try_into().unwrap()))
            .collect();

        Ok(Self { page, last, list_of_universes })
    }

    fn size(&self) -> usize {
        8 + self.list_of_universes.len() * 2
    }
}
