mod error;
mod packet;
mod source;

pub use error::*;
pub use packet::*;
pub use source::*;

pub const PORT: u16 = 6454;

/// The Style code defines the general functionality of a Controller.
/// The Style code is returned in [`ArtPollReply`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(facet::Facet)]
#[repr(u8)]
pub enum StyleCode {
    /// A DMX to/from Art-Net device.
    #[default]
    StNode = 0x00,
    /// A lighting console.
    StController = 0x01,
    /// A Media Server.
    StMedia = 0x02,
    /// A network routing device.
    StRoute = 0x03,
    /// A backup device.
    StBackup = 0x04,
    /// A configuration or diagnostic tool.
    StConfig = 0x05,
    /// A visualiser.
    StVisual = 0x06,
}

impl TryFrom<u8> for StyleCode {
    type Error = crate::Error;

    fn try_from(value: u8) -> crate::Result<Self> {
        match value {
            0x00 => Ok(StyleCode::StNode),
            0x01 => Ok(StyleCode::StController),
            0x02 => Ok(StyleCode::StMedia),
            0x03 => Ok(StyleCode::StRoute),
            0x04 => Ok(StyleCode::StBackup),
            0x05 => Ok(StyleCode::StConfig),
            0x06 => Ok(StyleCode::StVisual),
            _ => Err(crate::Error::InvalidStyleCode(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(facet::Facet)]
pub struct PortAddress(u16);

impl PortAddress {
    pub const MIN: Self = PortAddress(0);
    pub const MAX: Self = PortAddress(32767);

    pub fn new(net: u8, sub_net: u8, universe: u8) -> Result<Self> {
        if net > 127 {
            return Err(Error::InvalidNet);
        }
        if sub_net > 15 {
            return Err(Error::InvalidSubNet);
        }
        if universe > 15 {
            return Err(Error::InvalidUniverse);
        }

        let address = ((net as u16) << 8) | ((sub_net as u16) << 4) | (universe as u16);
        Ok(PortAddress(address))
    }

    pub fn from_raw(address: u16) -> Result<Self> {
        if address < 32768 { Ok(PortAddress(address)) } else { Err(Error::InvalidPortAddress) }
    }

    pub fn net(&self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn sub_net(&self) -> u8 {
        ((self.0 >> 4) & 0x0F) as u8
    }

    pub fn universe(&self) -> u8 {
        (self.0 & 0x0F) as u8
    }

    pub fn get(&self) -> u16 {
        self.0
    }
}

impl TryFrom<u16> for PortAddress {
    type Error = crate::Error;

    fn try_from(value: u16) -> Result<Self> {
        PortAddress::from_raw(value)
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
