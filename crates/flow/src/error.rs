#[derive(Debug, thiserror::Error)]
pub enum FlowError {
    #[error("Failed to cast value")]
    CastFailed,
}
