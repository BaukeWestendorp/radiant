use super::{AssetId, FixtureGroup, effect_graph::EffectGraph};

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Default)]
pub struct Cue {
    pub effect_graph: Option<AssetId<EffectGraph>>,
    pub fixture_group: Option<AssetId<FixtureGroup>>,
}
