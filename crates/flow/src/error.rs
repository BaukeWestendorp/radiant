use super::NodeId;

#[derive(Debug, Clone, thiserror::Error)]
pub enum FlowError {
    #[error("Node {0:?} has no socket named {1}")]
    NoSocketNamed(NodeId, String),
    #[error("Failed to cast to target type")]
    CastFailed,
}

pub type Result<T> = std::result::Result<T, FlowError>;
