use std::collections::HashMap;

use cue::Cue;
use effect_graph::EffectGraph;
use fixture_group::FixtureGroup;
use presets::DimmerPreset;

pub mod cue;
pub mod effect_graph;
pub mod fixture_group;
pub mod presets;

pub type AssetId = u32;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Default)]
pub struct Assets {
    pub effect_graphs: AssetPool<EffectGraph>,
    pub fixture_groups: AssetPool<FixtureGroup>,
    pub dimmer_presets: AssetPool<DimmerPreset>,
    pub cues: AssetPool<Cue>,
}

pub type AssetPool<T> = HashMap<AssetId, Asset<T>>;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Default)]
pub struct Asset<T> {
    pub label: String,
    pub id: AssetId,
    pub data: T,
}
