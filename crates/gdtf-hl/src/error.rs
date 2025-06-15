use crate::{Attribute, DmxMode, FixtureId};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(
        "Tried to set attribute '{attribute}' for fixture with id '{fixture_id:?}', which does not support that attribute"
    )]
    InvalidAttributeForFixture { attribute: Attribute, fixture_id: FixtureId },
    #[error("Tried to get DMX mode '{dmx_mode}' for fixture type '{fixture_type_name}")]
    InvalidDmxMode { dmx_mode: DmxMode, fixture_type_name: String },
}
