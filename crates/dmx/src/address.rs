use crate::DmxChannel;
use anyhow::Context;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DmxAddress {
    pub universe: u16,
    pub channel: DmxChannel,
}

impl DmxAddress {
    pub fn new(universe: u16, channel: DmxChannel) -> Self {
        Self { universe, channel }
    }

    pub fn absolute_address(&self) -> u32 {
        (self.universe * 512 + self.channel.value()) as u32
    }
}

impl FromStr for DmxAddress {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Input must contain exactly one dot ('.')"));
        }

        let universe = parts[0]
            .parse::<u16>()
            .context("Failed to parse universe")?;
        let channel = parts[1].parse::<u16>().context("Failed to parse channel")?;

        Ok(DmxAddress::new(universe, DmxChannel::new(channel)?))
    }
}

impl Display for DmxAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.universe, self.channel)
    }
}

impl<'de> serde::Deserialize<'de> for DmxAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for DmxAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self)
    }
}
