use super::AssetId;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Default)]
pub struct Sequence {
    pub cues: Vec<AssetId>,
}
