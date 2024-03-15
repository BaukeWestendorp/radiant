#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    ParseError(String),
}

impl Error {
    pub fn message(&self) -> String {
        match self {
            Error::ParseError(message) => message.clone(),
        }
    }
}
