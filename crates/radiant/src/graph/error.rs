use super::{NodeId, OutputId};

#[derive(Debug, Clone, thiserror::Error)]
pub enum GraphError {
    #[error("Node {0:?} has no socket named {1}")]
    NoSocketNamed(NodeId, String),
    #[error("Output with id {0:?} has no cached output value.")]
    NoCachedOutputValueFor(OutputId),
    #[error("Failed to cast to target type")]
    CastFailed,
}
