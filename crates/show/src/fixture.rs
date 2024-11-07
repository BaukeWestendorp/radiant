use flow::error::GraphError;
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
pub struct FixtureId(pub u32);

impl FromStr for FixtureId {
    type Err = GraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse().map_err(|_| GraphError::ParseFailed)?))
    }
}

impl FixtureId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn id(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Fixture {
    id: FixtureId,
}

impl Fixture {
    pub fn new(id: FixtureId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &FixtureId {
        &self.id
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
}

impl Default for AttributeValue {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}
