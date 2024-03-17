#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    UnzipError(String),
    ParseError(String),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::UnzipError(message) => message.clone(),
            Error::ParseError(message) => message.clone(),
        }
    }
}
