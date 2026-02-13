use crate::lua::FixtureId;

#[derive(Debug, Clone)]
pub enum Command {
    SetAttributeValue(SetAttributeValue),
}

#[derive(Debug, Clone)]
pub struct SetAttributeValue {
    pub fixture_id: FixtureId,
    pub attribute: String,
    pub value: f32,
}
