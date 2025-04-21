use std::collections::HashMap;

use crate::assets::EffectGraph;

pub type AssetId = u32;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Default)]
pub struct Assets {
    pub effect_graphs: AssetPool<EffectGraph>,
}

pub type AssetPool<T> = HashMap<AssetId, Asset<T>>;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Default)]
pub struct Asset<T> {
    pub id: AssetId,
    pub data: T,
}
