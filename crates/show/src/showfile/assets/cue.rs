use super::AssetId;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Default)]
pub struct Cue {
    pub effect_graph: AssetId,
    pub fixture_group: AssetId,
}
