use std::collections::HashMap;

use crate::backend::patch::{
    attr::{Attribute, AttributeValue},
    fixture::FixtureId,
};

/// A collection of attribute values, either connected to specific fixtures, fixture types, or generic attributes.
#[derive(Debug, Clone, PartialEq)]
pub enum Preset {
    Selective(SelectivePreset),
}

/// A preset that has attribute values for specific fixures.
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
