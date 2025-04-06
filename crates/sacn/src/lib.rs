//! # sACN
//! This library provides a Rust interface for working with sACN.
//!
//! # Features
//! TODO: List features.

mod error;
pub mod source;

mod packet;

pub use error::Error;

pub type ComponentIdentifier = uuid::Uuid;
