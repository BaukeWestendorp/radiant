//! This crate provides a simple DMX library for Rust. It allows you to create, manipulate, and output DMX universes and its channels.

#![warn(missing_docs)]

pub mod address;
pub mod channel;
pub mod error;
pub mod output;
pub mod universe;

pub use address::*;
pub use channel::*;
pub use error::*;
pub use output::*;
pub use universe::*;

/// The number of channels in a DMX universe.
pub const UNIVERSE_SIZE: u16 = 512;
