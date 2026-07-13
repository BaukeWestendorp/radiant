use crate::PortProtocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortDirection {
    Input,
    Output,
    Bidirectional,
}

#[derive(Debug, Clone)]
pub struct Port {
    pub direction: PortDirection,
    pub protocol: PortProtocol,
    pub input_universe: Option<u8>,
    pub output_universe: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(facet::Facet)]
#[facet(transparent)]
#[repr(transparent)]
pub struct PortId(u16);

impl PortId {
    pub fn new(id: u16) -> Self {
        Self(id)
    }

    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(facet::Facet)]
#[facet(transparent)]
#[facet(facet_validate::min = 0, facet_validate::max = 32767)]
#[repr(transparent)]
pub struct PortAddress(u16);

impl PortAddress {
    pub const MIN: Self = PortAddress(0);
    pub const MAX: Self = PortAddress(32767);

    pub fn new(net: NetId, sub_net: SubNetId, universe: UniverseId) -> Self {
        let address = ((net.as_u8() as u16) << 8)
            | ((sub_net.as_u8() as u16) << 4)
            | (universe.as_u8() as u16);
        PortAddress(address)
    }

    pub fn from_raw(address: u16) -> crate::Result<Self> {
        if address < 32768 {
            Ok(PortAddress(address))
        } else {
            Err(crate::Error::InvalidPortAddress)
        }
    }

    pub fn net(&self) -> NetId {
        NetId((self.0 >> 8) as u8)
    }

    pub fn sub_net(&self) -> SubNetId {
        SubNetId(((self.0 >> 4) & 0x0F) as u8)
    }

    pub fn universe(&self) -> UniverseId {
        UniverseId((self.0 & 0x0F) as u8)
    }

    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

impl TryFrom<u16> for PortAddress {
    type Error = crate::Error;

    fn try_from(value: u16) -> crate::Result<Self> {
        PortAddress::from_raw(value)
    }
}

impl Default for PortAddress {
    fn default() -> Self {
        // Let's use 1 as a default, as a universe of 0
        // is deprecated in Art-Net 4 for better sACN compatibility.
        Self(1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(facet::Facet)]
#[facet(transparent)]
#[facet(facet_validate::min = 0, facet_validate::max = 127)]
#[repr(transparent)]
pub struct NetId(u8);

impl NetId {
    pub const MIN: Self = NetId(0);
    pub const MAX: Self = NetId(127);

    pub fn new(value: u8) -> crate::Result<Self> {
        if value <= 127 { Ok(NetId(value)) } else { Err(crate::Error::InvalidNetId) }
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for NetId {
    type Error = crate::Error;

    fn try_from(value: u8) -> crate::Result<Self> {
        NetId::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(facet::Facet)]
#[facet(transparent)]
#[facet(facet_validate::min = 0, facet_validate::max = 15)]
#[repr(transparent)]
pub struct SubNetId(u8);

impl SubNetId {
    pub const MIN: Self = SubNetId(0);
    pub const MAX: Self = SubNetId(15);

    pub fn new(value: u8) -> crate::Result<Self> {
        if value <= 15 { Ok(SubNetId(value)) } else { Err(crate::Error::InvalidSubNetId) }
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for SubNetId {
    type Error = crate::Error;

    fn try_from(value: u8) -> crate::Result<Self> {
        SubNetId::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(facet::Facet)]
#[facet(transparent)]
#[facet(facet_validate::min = 0, facet_validate::max = 15)]
#[repr(transparent)]
pub struct UniverseId(u8);

impl UniverseId {
    pub const MIN: Self = UniverseId(0);
    pub const MAX: Self = UniverseId(15);

    pub fn new(value: u8) -> crate::Result<Self> {
        if value <= 15 { Ok(UniverseId(value)) } else { Err(crate::Error::InvalidUniverseId) }
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for UniverseId {
    type Error = crate::Error;

    fn try_from(value: u8) -> crate::Result<Self> {
        UniverseId::new(value)
    }
}
