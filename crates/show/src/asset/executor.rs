use super::{AssetId, Sequence};

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Default)]
pub struct Executor {
    pub sequence: Option<AssetId<Sequence>>,
    pub current_index: Option<usize>,
}
