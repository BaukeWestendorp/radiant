use anyhow::{anyhow, bail};
use std::str::FromStr;

#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct DmxChannel(u16);

impl DmxChannel {
    pub fn new(value: u16) -> anyhow::Result<Self> {
        if value > 512 {
            bail!("value must be between 0 and 512");
        }

        Ok(Self(value))
    }

    pub fn set_value(&mut self, value: u16) -> anyhow::Result<()> {
        if value > 512 {
            bail!("value must be between 0 and 512");
        }

        self.0 = value;

        Ok(())
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}

impl FromStr for DmxChannel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(
            s.parse()
                .map_err(|_| anyhow!("Failed to parse DMX Channel"))?,
        )
    }
}
