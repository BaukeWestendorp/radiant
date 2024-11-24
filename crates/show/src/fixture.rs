use std::{fmt::Display, str::FromStr};

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
pub struct FixtureId(pub u32);

impl FixtureId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn id(&self) -> u32 {
        self.0
    }
}

impl FromStr for FixtureId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl Display for FixtureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct AttributeValue {
    value: f32,
}

impl AttributeValue {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
        }
    }

    pub fn relative_value(&self) -> f32 {
        self.value
    }

    pub fn byte(&self) -> u8 {
        (self.value * 255f32) as u8
    }

    pub fn inverted(&self) -> AttributeValue {
        Self::new(1.0 - self.relative_value())
    }
}

impl Default for AttributeValue {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}
