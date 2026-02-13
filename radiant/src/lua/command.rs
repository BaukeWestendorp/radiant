use crate::lua::FixtureId;

pub enum Command {
    SetAttributeValue { fixture_id: FixtureId, attribute: String, value: f32 },
}
