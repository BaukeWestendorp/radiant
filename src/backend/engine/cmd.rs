use crate::backend::object::Object;
use crate::backend::patch::attr::Attribute;
use crate::backend::patch::attr::AttributeValue;
use crate::backend::patch::fixture::DmxMode;
use crate::backend::patch::fixture::FixtureId;
use crate::backend::preset::PresetContent;
use crate::dmx;

/// A [Command] is the interface between the engine and the backend.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    PatchFixture { id: FixtureId, address: dmx::Address, dmx_mode: DmxMode, gdtf_file_name: String },
    SetDmxValue { address: dmx::Address, value: dmx::Value },
    SetAttributeValue { fixture_id: FixtureId, attribute: Attribute, value: AttributeValue },
    SetPreset { preset: PresetContent },
    New(Object),
}
