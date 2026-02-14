use std::{io, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("could not find project.json in {0:?}")]
    ProjectJsonNotFound(PathBuf),
    #[error("failed to parse {0:?}: {1}")]
    ParseError(PathBuf, serde_json::Error),

    #[error("i/o error: {0}")]
    Io(#[from] io::Error),
    #[error("zeevonk error: {0}")]
    Zeevonk(#[from] zeevonk::Error),
    #[error("serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("object error: {0}")]
    Object(#[from] crate::object::Error),
}
