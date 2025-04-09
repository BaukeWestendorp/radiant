#![warn(missing_docs)]

//! # sACN
//! This library provides a Rust interface for working with sACN.
//!
//! # Features
//! TODO: List features.

mod error;
pub mod packet;
pub mod receiver;
pub mod source;

pub use error::Error;

use uuid::Uuid;

/// A CID ([ComponentIdentifier]) is a [Uuid].
///
/// Each piece of equipment should maintain the same CID for
/// its entire lifetime (e.g. by storing it in read-only memory).
/// This means that a particular component on the network can be identified
/// as the same entity from day to day despite network
/// interruptions, power down, or other disruptions.
///
/// However, in some systems there may be situations in which volatile components
/// are dynamically created "on the fly" and,
/// in these cases, the controlling process can generate CIDs as required.
/// The choice of UUIDs for CIDs allows them to be generated as required
/// without reference to any registration process or authority.
pub type ComponentIdentifier = Uuid;

/// The default port for sACN.
pub const DEFAULT_PORT: u16 = 5568;

/// The universe number on which discovery packets will be sent.
pub const DISCOVERY_UNIVERSE: u32 = 64214;

/// The maximum size of a universe.
pub const MAX_UNIVERSE_SIZE: u16 = 512;
