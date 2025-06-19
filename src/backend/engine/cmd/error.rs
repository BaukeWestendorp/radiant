#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unexpected End of Line")]
    UnexpectedEOL,
    #[error("Expected End of Line")]
    ExpectedEOL,
    #[error("Unexpected token: '{0}'")]
    UnexpectedToken(String),
    #[error("Expected Object Id")]
    ExpectedObjectId,
    #[error("A negative Object Id is not allowed")]
    NegativeObjectId,
    #[error("Expected an identifier")]
    ExpectedIdent,
    #[error("Expected a string")]
    ExpectedString,
    #[error("Expected an integer")]
    ExpectedInteger,
}
