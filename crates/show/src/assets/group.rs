use crate::FixtureId;

super::asset_id!(pub GroupId);

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub id: GroupId,
    pub label: String,
    pub fixtures: Vec<FixtureId>,
}

impl super::Asset for Group {
    type Id = GroupId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl From<showfile::Group> for Group {
    fn from(group: showfile::Group) -> Self {
        Self {
            id: GroupId(group.id),
            label: group.label,
            fixtures: group.fixtures.into_iter().map(FixtureId).collect(),
        }
    }
}
