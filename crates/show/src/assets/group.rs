use crate::{showfile, FixtureId};

super::asset_id!(pub GroupId);

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub id: GroupId,
    pub label: String,
    pub fixtures: Vec<FixtureId>,
}

impl Group {
    pub fn len(&self) -> usize {
        self.fixtures.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fixtures.is_empty()
    }
}

impl super::Asset for Group {
    type Id = GroupId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn label(&self) -> &str {
        &self.label
    }
}

impl Group {
    pub(crate) fn from_showfile(group: showfile::Group) -> Self {
        Self {
            id: GroupId(group.id),
            label: group.label,
            fixtures: group.fixtures.into_iter().map(FixtureId).collect(),
        }
    }

    pub(crate) fn to_showfile(&self) -> showfile::Group {
        showfile::Group {
            id: self.id.0,
            label: self.label.clone(),
            fixtures: self.fixtures.iter().map(|f| f.0).collect(),
        }
    }
}
