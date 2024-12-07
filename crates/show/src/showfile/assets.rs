use flow::Point;

use crate::FlowEffectGraph;

pub type AssetId = u32;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Assets {
    pub groups: Vec<Group>,
    pub effect_graphs: Vec<EffectGraph>,
    pub cuelists: Vec<CueList>,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Group {
    pub id: AssetId,
    pub label: String,
    pub fixtures: Vec<super::FixtureId>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct EffectGraph {
    pub id: AssetId,
    pub label: String,
    pub graph: FlowEffectGraph,
    pub offset: Point,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct CueList {
    pub id: AssetId,
    pub label: String,
    pub cues: Vec<Cue>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Cue {
    pub label: String,
    pub lines: Vec<CueLine>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct CueLine {
    pub label: String,
    pub group: AssetId,
    pub effect: Effect,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Effect {
    Graph(AssetId),
}
