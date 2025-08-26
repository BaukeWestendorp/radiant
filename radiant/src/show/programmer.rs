use std::collections::HashMap;

use crate::show::{Attribute, AttributeValue, FixtureId};

/// Contains 'work in progress' values that can be stored into presets.
#[derive(Debug, Default)]
pub struct Programmer {
    values: HashMap<(FixtureId, Attribute), AttributeValue>,
}

impl Programmer {
    /// Sets an [AttributeValue] for a given (main) attribute [Attribute] on the
    /// fixture with the given [FixtureId]. The attribute has to be a main
    /// attribute to prevent double channel assignments.
    pub fn set_value(&mut self, fid: FixtureId, main_attribute: Attribute, value: AttributeValue) {
        self.values.insert((fid, main_attribute), value);
    }

    /// Gets an [AttributeValue] for the given (main) [Attribute] on the
    /// fixture with the given [FixtureId].
    pub fn value(&self, fid: FixtureId, main_attribute: Attribute) -> Option<AttributeValue> {
        self.values.get(&(fid, main_attribute)).copied()
    }

    /// Gets an iterator over all values.
    pub fn values(&self) -> impl IntoIterator<Item = (&FixtureId, &Attribute, &AttributeValue)> {
        self.values.iter().map(|((fid, attr), value)| (fid, attr, value))
    }

    /// Clears all values.
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Returns `true` if there are no values stored in the programmer.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
