mod error;
mod packet;
mod source;

pub use error::*;
pub use packet::*;
pub use source::*;

pub const PORT: u16 = 6454;

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

    pub fn from_raw(address: u16) -> Result<Self> {
        if address < 32768 { Ok(PortAddress(address)) } else { Err(Error::InvalidPortAddress) }
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

    fn try_from(value: u16) -> Result<Self> {
        PortAddress::from_raw(value)
    }
}

impl Default for PortAddress {
    fn default() -> Self {
        // Let's use 1 as a default, as a universe of 0 is deprecated in Art-Net 4 for better sACN compatibility.
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

    pub fn new(value: u8) -> Result<Self> {
        if value <= 127 { Ok(NetId(value)) } else { Err(Error::InvalidNetId) }
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for NetId {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self> {
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

    pub fn new(value: u8) -> Result<Self> {
        if value <= 15 { Ok(SubNetId(value)) } else { Err(Error::InvalidSubNetId) }
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for SubNetId {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self> {
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

    pub fn new(value: u8) -> Result<Self> {
        if value <= 15 { Ok(UniverseId(value)) } else { Err(Error::InvalidUniverseId) }
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for UniverseId {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self> {
        UniverseId::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(facet::Facet)]
pub struct Universe {
    channels: [u8; 512],
}

impl Universe {
    pub fn new() -> Self {
        Self { channels: [0; 512] }
    }

    pub fn get_channel(&self, ix: usize) -> Result<u8> {
        self.channels.get(ix).copied().ok_or(Error::ChannelOutOfBounds)
    }

    pub fn set_channel(&mut self, ix: usize, value: u8) -> Result<()> {
        if let Some(channel) = self.channels.get_mut(ix) {
            *channel = value;
            Ok(())
        } else {
            Err(Error::ChannelOutOfBounds)
        }
    }

    pub fn as_bytes(&self) -> &[u8; 512] {
        &self.channels
    }
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(facet::Facet)]
pub struct Kiloverse {
    universes: Box<[Universe; 1024]>,
}

impl Kiloverse {
    pub fn new() -> Self {
        Self { universes: vec![Universe::new(); 1024].into_boxed_slice().try_into().unwrap() }
    }

    pub fn get_universe(&self, ix: usize) -> Result<&Universe> {
        self.universes.get(ix).ok_or(Error::UniverseOutOfBounds)
    }

    pub fn get_universe_mut(&mut self, ix: usize) -> Result<&mut Universe> {
        self.universes.get_mut(ix).ok_or(Error::UniverseOutOfBounds)
    }
}

impl Default for Kiloverse {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(facet::Facet)]
#[repr(transparent)]
pub struct FixedString<const N: usize>([u8; N]);

impl<const N: usize> FixedString<N> {
    pub fn try_from_str(s: &str) -> crate::Result<Self> {
        let mut buf = [0u8; N];
        let bytes = s.as_bytes();

        if bytes.len() >= N {
            return Err(crate::Error::StringTooLong);
        }

        buf[..bytes.len()].copy_from_slice(bytes);
        Ok(Self(buf))
    }

    pub fn as_str(&self) -> &str {
        let null_pos = self.0.iter().position(|&b| b == 0).unwrap_or(N);
        std::str::from_utf8(&self.0[..null_pos]).unwrap_or("")
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> Default for FixedString<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> TryFrom<&[u8]> for FixedString<N> {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() > N {
            return Err(crate::Error::StringTooLong);
        }

        let mut buf = [0u8; N];
        buf[..value.len()].copy_from_slice(value);
        Ok(Self(buf))
    }
}
