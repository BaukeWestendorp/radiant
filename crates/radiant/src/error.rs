/// A type alias for errors using the `eyre` crate.
pub type Error = eyre::Error;

/// An type alias for `Result<T, Error>` using the `eyre` crate.
pub type Result<T> = eyre::Result<T>;
