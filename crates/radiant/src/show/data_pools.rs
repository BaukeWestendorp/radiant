use std::collections::HashMap;

use gpui::SharedString;

use super::patch::FixtureId;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DataPools {
    fixture_groups: HashMap<FixtureGroupId, FixtureGroup>,
}

impl DataPools {
    pub fn new() -> Self {
        Self {
            fixture_groups: HashMap::new(),
        }
    }

    pub fn add_fixture_group(&mut self, fixture_group: FixtureGroup) -> FixtureGroupId {
        let id = self.get_new_fixture_group_id();
        self.fixture_groups.insert(id, fixture_group);
        id
    }

    pub fn set_fixture_group(&mut self, id: FixtureGroupId, fixture_group: FixtureGroup) {
        self.fixture_groups.insert(id, fixture_group);
    }

    pub fn fixture_group(&self, id: FixtureGroupId) -> Option<&FixtureGroup> {
        self.fixture_groups.get(&id)
    }

    pub fn fixture_group_mut(&mut self, id: FixtureGroupId) -> Option<&mut FixtureGroup> {
        self.fixture_groups.get_mut(&id)
    }

    pub fn fixture_groups(&self) -> impl Iterator<Item = (FixtureGroupId, &FixtureGroup)> {
        self.fixture_groups.iter().map(|(id, preset)| (*id, preset))
    }

    fn get_new_fixture_group_id(&self) -> FixtureGroupId {
        // TODO: This is not a good way to get a new id. This only works if you can't
        // remove fixture groups.
        FixtureGroupId(self.fixture_groups.len() as usize)
    }
}

pub trait DataPool {
    fn label(&self) -> &str;

    fn set_label(&mut self, label: &str);
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FixtureGroup {
    label: SharedString,
    pub fixtures: Vec<FixtureId>,
}

impl FixtureGroup {
    pub fn new(label: &str, fixtures: Vec<FixtureId>) -> Self {
        Self {
            label: label.to_string().into(),
            fixtures,
        }
    }
}

impl DataPool for FixtureGroup {
    fn label(&self) -> &str {
        &self.label
    }

    fn set_label(&mut self, label: &str) {
        self.label = label.to_string().into();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct FixtureGroupId(pub usize);
