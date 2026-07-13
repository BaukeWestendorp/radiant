mod error;
mod node;
mod packet;

pub use error::*;
pub use node::*;
pub use packet::*;

pub const PORT: u16 = 6454;

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
