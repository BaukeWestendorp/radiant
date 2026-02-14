use std::collections::BTreeMap;
use std::sync::RwLock;

use zeevonk::{
    project::{FixtureId, IntoFixtureId},
    value::AttributeValues,
};

use crate::parameter::Parameter;

pub struct Programmer {
    // Stores the current values being programmed for each fixture.
    programmed_values: RwLock<AttributeValues>,

    // Tracks which parameters are "touched" or "active" for each fixture.
    // This is used to know what will be stored or cleared.
    touched_parameters: RwLock<BTreeMap<FixtureId, Vec<Parameter>>>,
}

impl Programmer {
    pub fn new() -> Self {
        Self {
            programmed_values: RwLock::new(AttributeValues::new()),
            touched_parameters: RwLock::new(BTreeMap::new()),
        }
    }

    /// Returns a clone of the programmed attribute values.
    pub fn programmed_values(&self) -> AttributeValues {
        self.programmed_values.read().unwrap().clone()
    }

    /// Returns a clone of the touched parameters map.
    pub fn touched_parameters(&self) -> BTreeMap<FixtureId, Vec<Parameter>> {
        self.touched_parameters.read().unwrap().clone()
    }

    /// Sets a value for a fixture and parameter in the programmer.
    pub fn set_parameter(&self, fixture_id: impl IntoFixtureId, parameter: Parameter) {
        let Some(fixture_id) = fixture_id.into_fixture_id() else { return };

        {
            let mut programmed_values = self.programmed_values.write().unwrap();
            for (attribute, value) in parameter.to_attribute_values() {
                programmed_values.set(fixture_id, attribute, value);
            }
        }

        // Mark this parameter as touched
        {
            let mut touched_parameters = self.touched_parameters.write().unwrap();
            touched_parameters.entry(fixture_id).or_insert_with(Vec::new).push(parameter);
        }
    }

    /// Clears all values from the programmer.
    pub fn clear(&self) {
        self.programmed_values.write().unwrap().clear();
        self.touched_parameters.write().unwrap().clear();
    }
}
