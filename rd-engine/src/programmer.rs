use crate::{
    gdtf::attr::AttributeName,
    patch::FixtureId,
    value::{AttributeValue, AttributeValues},
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Programmer {
    values: AttributeValues,
}

impl Programmer {
    pub fn new() -> Self {
        Self { values: AttributeValues::new() }
    }

    pub fn set(&mut self, fixture_id: FixtureId, attribute: AttributeName, value: AttributeValue) {
        self.values.set(fixture_id, attribute, value);
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn values(&self) -> &AttributeValues {
        &self.values
    }
}
