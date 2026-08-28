#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PatchConfig {
    pub fixtures: Vec<FixtureConfig>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FixtureConfig {
    pub id: u32,
    pub name: String,
    pub dmx_address: rd_dmx::Address,
    pub fixture_kind: FixtureKind,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FixtureKind {
    pub fixture_type_id: FixtureTypeId,
    pub dmx_mode: String,
}

impl std::fmt::Display for FixtureKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.fixture_type_id, self.dmx_mode)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct FixtureTypeId(uuid::Uuid);

impl std::ops::Deref for FixtureTypeId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for FixtureTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
