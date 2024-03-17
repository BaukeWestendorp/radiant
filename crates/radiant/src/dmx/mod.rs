use std::fmt::Display;
use std::str::FromStr;

pub mod color;

#[derive(Debug, Clone, Default)]
pub struct DmxOutput(Vec<DmxUniverse>);

impl DmxOutput {
    pub fn new() -> Self {
        DmxOutput(Vec::new())
    }

    pub fn set_channel(&mut self, channel: DmxChannel, value: u8) {
        if let Some(universe) = self.get_universe_mut(channel.universe) {
            universe.set_channel(channel.address, value);
        } else {
            log::warn!(
                "Tried to set channel {} in universe {} but it does not exist",
                channel.address,
                channel.universe
            );
        }
    }

    pub fn get_channel(&self, channel: DmxChannel) -> Option<u8> {
        self.get_universe(channel.universe)
            .map(|u| u.channels[channel.address as usize])
    }

    pub fn add_universe_if_absent(&mut self, universe: DmxUniverse) {
        if self.get_universe(universe.id).is_some() {
            return;
        }
        self.0.push(universe);
    }

    pub fn get_universe(&self, id: u32) -> Option<&DmxUniverse> {
        self.0.iter().find(|u| u.id == id)
    }

    pub fn get_universe_mut(&mut self, id: u32) -> Option<&mut DmxUniverse> {
        self.0.iter_mut().find(|u| u.id == id)
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        for universe in &self.0 {
            buffer.extend_from_slice(&universe.channels);
        }
        buffer
    }
}

#[derive(Debug, Clone)]
pub struct DmxUniverse {
    id: u32,
    channels: [u8; 512],
}

impl DmxUniverse {
    pub fn new(id: u32) -> Result<Self, anyhow::Error> {
        if id == 0 {
            return Err(anyhow::anyhow!("Universes must have an id greater than 0"));
        }

        Ok(DmxUniverse {
            id,
            channels: [0; 512],
        })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn set_channel(&mut self, channel: u16, value: u8) {
        if channel == 0 || channel > 512 {
            log::warn!(
                "Tried to set channel {} in universe {} but it is out of range",
                channel,
                self.id
            );
            return;
        }

        self.channels[channel as usize - 1] = value;
    }

    pub fn get_channel(&self, channel: u16) -> Option<u8> {
        if channel == 0 || channel > 512 {
            log::warn!(
                "Tried to get channel {} in universe {} but it is out of range",
                channel - 1,
                self.id
            );
            return None;
        }

        Some(self.channels[channel as usize - 1])
    }

    pub fn get_channels(&self) -> &[u8; 512] {
        &self.channels
    }
}

#[derive(Debug, Clone)]
pub struct DmxChannel {
    pub universe: u32,
    pub address: u16,
}

impl DmxChannel {
    pub fn new(universe: u32, address: u16) -> Result<Self, anyhow::Error> {
        if universe == 0 {
            return Err(anyhow::anyhow!("Universe must be greater than 0"));
        }

        if address == 0 || address > 512 {
            return Err(anyhow::anyhow!("Channel must be less than 512"));
        }

        Ok(DmxChannel { universe, address })
    }
}

impl<'de> serde::Deserialize<'de> for DmxChannel {
    fn deserialize<D>(deserializer: D) -> Result<DmxChannel, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, serde::Deserialize)]
        struct Intermediate {
            universe: u32,
            address: u16,
        }

        let intermediate: Intermediate = serde::Deserialize::deserialize(deserializer)?;

        DmxChannel::new(intermediate.universe, intermediate.address)
            .map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for DmxChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl FromStr for DmxChannel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid DMX channel string: {}", s));
        }

        let universe = parts[0].parse()?;
        let channel = parts[1].parse()?;

        Ok(DmxChannel {
            universe,
            address: channel,
        })
    }
}

impl Display for DmxChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{:03}", self.universe, self.address)
    }
}
