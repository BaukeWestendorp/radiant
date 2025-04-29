use std::collections::HashMap;

use effect_graph::EffectGraph;
use fixture_group::FixtureGroup;

pub mod effect_graph;
pub mod fixture_group;

pub type AssetId = u32;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Default)]
pub struct Assets {
    pub effect_graphs: AssetPool<EffectGraph>,
    pub fixture_groups: AssetPool<FixtureGroup>,
}

pub type AssetPool<T> = HashMap<AssetId, Asset<T>>;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Default)]
pub struct Asset<T> {
    pub id: AssetId,
    pub data: T,
}
