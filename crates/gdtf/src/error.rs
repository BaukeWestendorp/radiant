#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    UnzipError,
    ParseError(String),
    InvalidDataVersion(String),
    MissingDescription,
    InvalidName(String),
    EmptyNode,
    InvalidGuid(String),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::UnzipError => "Failed to unzip".to_string(),
            Error::ParseError(message) => format!("Parse error: {}", message),
            Error::InvalidDataVersion(str) => format!("Invalid data version: {}", str),
            Error::MissingDescription => "Missing description.xml file in the archive".to_string(),
            Error::InvalidName(str) => format!("Invalid name: '{}'", str),
            Error::EmptyNode => "Empty node".to_string(),
            Error::InvalidGuid(str) => format!("Invalid GUID: '{}'", str),
        }
    }
}
