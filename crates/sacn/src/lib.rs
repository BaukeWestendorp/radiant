#![warn(missing_docs)]

//! # sACN
//! This library provides a Rust interface for working with sACN.
//!
//! # Features
//! TODO: List features.

use arrayvec::ArrayVec;

pub(crate) mod acn;
pub mod packet;
pub mod receiver;
pub mod source;

/// # E1.31 3.2 Universe
///
/// A set of up to 512 data slots identified by universe number.
/// Note: In E1.31 there may be multiple sources for a universe. See also: [Slot].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Universe {
    /// The universe number.
    pub number: UniverseNumber,
    /// The start code slot.
    pub start_code_slot: Slot,
    /// The data [Slot]s in the universe.
    pub data_slots: UniverseData,
}

/// A set of up to 512 data slots.
pub type UniverseData = ArrayVec<Slot, MAX_UNIVERSE_SIZE>;

impl Universe {
    /// Creates a new universe with the given number.
    pub fn new(number: UniverseNumber) -> Self {
        Universe { number, start_code_slot: 0, data_slots: ArrayVec::new() }
    }

    /// Creates a new universe with the given number and start code slot.
    pub fn with_start_code(number: UniverseNumber, start_code: Slot) -> Self {
        Universe { number, start_code_slot: start_code, data_slots: ArrayVec::new() }
    }

    /// Returns a the start code slot and data slots.
    pub fn slots(&self) -> ArrayVec<Slot, { 1 + MAX_UNIVERSE_SIZE }> {
        let mut slots = ArrayVec::new();
        slots.push(self.start_code_slot);
        slots.extend(self.data_slots.iter().copied());
        slots
    }
}

/// # E1.31 3.3 Universe Number.
///
/// Each E1.31 Data Packet contains a universe number identifying the universe it
/// carries. From an ACN perspective, a receiving device has some number of properties whose value is
/// addressed by the combination of a universe number and a data slot number. From an historical
/// perspective, a receiving device consumes some number of DMX512-A [DMX] data slots.
pub type UniverseNumber = u16;

/// # E1.31 3.4 Slot
pub type Slot = u8;

/// # E1.31 5.6 CID (Component Identifier)
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
pub type ComponentIdentifier = uuid::Uuid;

/// The default port for sACN.
pub const DEFAULT_PORT: u16 = 5568;

/// The universe number on which discovery packets will be sent.
pub const DISCOVERY_UNIVERSE: u32 = 64214;

/// The maximum size of a universe.
pub const MAX_UNIVERSE_SIZE: usize = 512;
