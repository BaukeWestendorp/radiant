use crate::UniverseId;

/// Error type for various error conditions that can occur during DMX operations.
#[derive(Debug)]
#[derive(facet::Facet)]
#[facet(derive(Error))]
#[repr(u8)]
pub enum Error {
    /// Channel has invalid value: '{0}', but should be in the range 1..=512"
    InvalidChannel(u16),
    /// Universe has invalid ID: '{0}'. Should be greater than 0"
    InvalidUniverseId(u16),
    /// Universe ID is out of the representable range after arithmetic operations: '{0}'
    UniverseIdOutOfRange(i64),
    /// Universe with ID '{0}' not found
    UniverseNotFound(UniverseId),

    /// Failed to parse channel: '{0}'
    ParseChannelFailed(String),
    /// Failed to parse universe ID: '{0}'
    ParseUniverseIdFailed(String),
    /// Failed to parse address: '{0}'
    ParseAddressFailed(String),
}
