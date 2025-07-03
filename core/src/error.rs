//! Error and result types for radiant-core.
//!
//! This module defines error and result types used throughout the
//! crate. All errors are based on the `eyre` crate, providing flexible and
//! user-friendly error handling for both library and application use.

/// A type alias for errors using the `eyre` crate.
pub type Error = eyre::Error;

/// An type alias for `Result<T, Error>` using the `eyre` crate.
pub type Result<T> = eyre::Result<T>;
