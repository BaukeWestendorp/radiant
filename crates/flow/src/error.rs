use crate::{NodeId, OutputId};

#[derive(Debug, thiserror::Error)]
pub enum FlowError {
    #[error("Node {0:?} has no socket named {1}")]
    NoSocketNamed(NodeId, String),
    #[error("Output with id {0:?} has no cached output value.")]
    NoCachedOutputValueFor(OutputId),
    #[error("Failed to cast to target type")]
    CastFailed,
}
