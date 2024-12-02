use crate::FlowEffectGraph;

pub type AssetId = u32;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Assets {
    pub groups: Vec<Group>,
    pub effects: Vec<Effect>,
    pub effect_graphs: Vec<EffectGraph>,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Group {
    pub id: AssetId,
    pub label: String,
    pub fixtures: Vec<super::FixtureId>,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Effect {
    pub id: AssetId,
    pub label: String,
    pub group: AssetId,
    pub kind: EffectKind,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum EffectKind {
    Graph(AssetId),
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct EffectGraph {
    pub id: AssetId,
    pub label: String,
    pub graph: FlowEffectGraph,
}
