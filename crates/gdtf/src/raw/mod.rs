pub mod attr_defs;
pub mod dmx_modes;
pub mod fixture_type;

pub use attr_defs::*;
pub use dmx_modes::*;
pub use fixture_type::*;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct RawGdtfDescription {
    #[serde(rename = "DataVersion")]
    pub data_version: String,

    #[serde(rename = "FixtureType")]
    pub fixture_type: RawFixtureType,
}

pub type RawName = String;

pub type RawNode = String;

pub type RawColorCIE = String;

pub type RawEnum = String;

pub type RawDmxValue = String;

pub type RawGuid = String;

pub type RawResource = String;
