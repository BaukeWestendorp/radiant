use crate::backend::object::Object;
use crate::backend::patch::attr::Attribute;
use crate::backend::patch::attr::AttributeValue;
use crate::backend::patch::fixture::DmxMode;
use crate::backend::patch::fixture::FixtureId;
use crate::backend::preset::PresetContent;
use crate::dmx;

/// A [Cmd] is the interface between the engine and the backend.
#[derive(Debug, Clone, PartialEq)]
pub enum Cmd {
    /// Patch a new fixture with the given details.
    PatchFixture { id: FixtureId, address: dmx::Address, dmx_mode: DmxMode, gdtf_file_name: String },
    /// Set a DMX value in the programmer.
    SetDmxValue { address: dmx::Address, value: dmx::Value },
    /// Set an attribute value in the programmer.
    SetAttributeValue { fixture_id: FixtureId, attribute: Attribute, value: AttributeValue },
    /// Set a preset in the programmer.
    SetPreset { preset: PresetContent },
    /// Creates a new [Object].
    New(Object),
}
