use std::collections::HashMap;

use crate::backend::patch::{
    attr::{Attribute, AttributeValue},
    fixture::FixtureId,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Preset {
    Selective(SelectivePreset),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SelectivePreset {
    attribute_values: HashMap<(FixtureId, Attribute), AttributeValue>,
}

impl SelectivePreset {
    pub fn get_attribute_values(
        &self,
    ) -> impl IntoIterator<Item = (&(FixtureId, Attribute), &AttributeValue)> {
        self.attribute_values.iter()
    }
}
