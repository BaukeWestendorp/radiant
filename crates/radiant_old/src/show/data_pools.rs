use std::collections::HashMap;

use gpui::SharedString;

use super::patch::FixtureId;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DataPools {
    groups: HashMap<usize, Group>,
}

impl DataPools {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
        }
    }

    pub fn add_group(&mut self, group: Group) -> usize {
        let id = self.get_new_group_id();
        self.groups.insert(id, group);
        id
    }

    pub fn set_group(&mut self, id: usize, group: Group) {
        self.groups.insert(id, group);
    }

    pub fn group(&self, id: usize) -> Option<&Group> {
        self.groups.get(&id)
    }

    pub fn group_mut(&mut self, id: usize) -> Option<&mut Group> {
        self.groups.get_mut(&id)
    }

    pub fn groups(&self) -> impl Iterator<Item = (usize, &Group)> {
        self.groups.iter().map(|(id, preset)| (*id, preset))
    }

    fn get_new_group_id(&self) -> usize {
        // TODO: This is not a good way to get a new id. This only works if you can't
        // remove fixture groups.
        self.groups.len() as usize
    }
}

pub trait DataPool {
    fn label(&self) -> &str;

    fn set_label(&mut self, label: &str);
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Group {
    label: SharedString,
    pub fixtures: Vec<FixtureId>,
}

impl Group {
    pub fn new(label: &str, fixtures: Vec<FixtureId>) -> Self {
        Self {
            label: label.to_string().into(),
            fixtures,
        }
    }
}

impl DataPool for Group {
    fn label(&self) -> &str {
        &self.label
    }

    fn set_label(&mut self, label: &str) {
        self.label = label.to_string().into();
    }
}
