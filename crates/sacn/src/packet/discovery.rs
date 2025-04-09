use super::{RootLayer, flags_and_length, source_name_from_str};
use crate::{ComponentIdentifier, Error, source::SourceConfig};

const VECTOR_EXTENDED_DISCOVERY: u32 = 0x00000002;
const VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST: u32 = 0x00000001;

/// Represents an E1.31 Universe Discovery Packet.
///
/// This packet contains a packed list of the universes upon which a source is actively operating.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniverseDiscoveryPacket {
    root: RootLayer,
    framing: FramingLayer,
    universe_discovery: UniverseDiscoveryLayer,
}

impl UniverseDiscoveryPacket {
    /// Creates a new [UniverseDiscoveryPacket].
    pub fn new(
        cid: ComponentIdentifier,
        source_name: &str,
        page: u8,
        last: u8,
        mut list_of_universes: Vec<u16>,
    ) -> Result<Self, Error> {
        list_of_universes.truncate(512);
        list_of_universes.sort();
        Ok(Self {
            root: RootLayer::new(cid, true),
            framing: FramingLayer::new(source_name)?,
            universe_discovery: UniverseDiscoveryLayer::new(page, last, list_of_universes),
        })
    }

    /// Creates a new [UniverseDiscoveryPacket] from a [SourceConfig].
    pub fn from_source_config(
        config: &SourceConfig,
        page: u8,
        last: u8,
        list_of_universes: Vec<u16>,
    ) -> Result<Self, Error> {
        Self::new(config.cid, &config.name, page, last, list_of_universes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            root: RootLayer::from_bytes(bytes)?,
            framing: FramingLayer::from_bytes(bytes)?,
            universe_discovery: UniverseDiscoveryLayer::from_bytes(bytes)?,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        vec![self.root.to_bytes(), self.framing.to_bytes(), self.universe_discovery.to_bytes()]
            .concat()
    }

    /// The [ComponentIdentifier] in this packet.
    pub fn cid(&self) -> &ComponentIdentifier {
        &self.root.cid
    }

    /// The source name in this packet.
    pub fn source_name(&self) -> &str {
        core::str::from_utf8(&self.framing.source_name).unwrap()
    }

    /// The page number in this packet.
    pub fn page(&self) -> u8 {
        self.universe_discovery.page
    }

    /// The last page number in this packet.
    pub fn last(&self) -> u8 {
        self.universe_discovery.last
    }

    /// The list of universes in this packet.
    pub fn list_of_universes(&self) -> &[u16] {
        &self.universe_discovery.list_of_universes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FramingLayer {
    source_name: [u8; 64],
}

impl FramingLayer {
    pub fn new(source_name: &str) -> Result<Self, Error> {
        let source_name = source_name_from_str(source_name)?;

        Ok(Self { source_name })
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        // E1.31 6.4.1 Universe Discovery Packet: Vector
        let vector = u32::from_be_bytes([bytes[40], bytes[41], bytes[42], bytes[43]]);
        if vector != VECTOR_EXTENDED_DISCOVERY {
            return Err(Error::InvalidFramingVector(vector));
        }

        // E1.31 6.4.2 Universe Discovery Packet: Source Name
        let source_name = bytes[44..108].try_into().unwrap();

        Ok(Self { source_name })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(38);
        bytes.extend(flags_and_length().to_be_bytes());
        bytes.extend(VECTOR_EXTENDED_DISCOVERY.to_be_bytes());
        bytes.extend(self.source_name);
        bytes.extend([0x00, 0x00, 0x00, 0x00]);
        bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UniverseDiscoveryLayer {
    /// Packet Number.
    page: u8,
    /// Final Page.
    last: u8,
    /// Sorted list of up to 512 16-bit universes.
    list_of_universes: Vec<u16>,
}

impl UniverseDiscoveryLayer {
    pub fn new(page: u8, last: u8, list_of_universes: Vec<u16>) -> Self {
        Self { page, last, list_of_universes }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        // E1.31 8.2 Universe Discovery Layer: Vector.
        let vector = u32::from_be_bytes([bytes[114], bytes[115], bytes[116], bytes[117]]);
        if vector != VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST {
            return Err(Error::InvalidUniverseDiscoveryUniverseListVector(vector));
        }

        let page = bytes[118];
        let last = bytes[119];
        let list_of_universes = bytes[120..]
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes(chunk.try_into().unwrap()))
            .collect();

        Ok(Self { page, last, list_of_universes })
    }

    pub fn to_bytes(&self, pdu_len: u16) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8);
        bytes.extend(flags_and_length(pdu_len).to_be_bytes());
        bytes.extend(VECTOR_UNIVERSE_DISCOVERY_UNIVERSE_LIST.to_be_bytes());
        bytes.push(self.page);
        bytes.push(self.last);
        bytes.extend(self.list_of_universes.iter().flat_map(|u| u.to_be_bytes()));
        bytes
    }
}
