use super::NodeId;

#[derive(Debug, Clone, thiserror::Error)]
pub enum GraphError {
    #[error("Node {0:?} has no socket named {1}")]
    NoSocketNamed(NodeId, String),
    #[error("Failed to cast to target type")]
    CastFailed,
    #[error("Failed to parse value as a String")]
    ParseFailed,
}
