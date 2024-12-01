use gpui::SharedString;

use crate::showfile::FixtureId;

super::asset_id!(pub GroupId);

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Group {
    id: GroupId,
    pub label: SharedString,
    pub fixtures: Vec<FixtureId>,
}

impl Group {
    pub(crate) fn new(id: GroupId, label: SharedString, fixtures: Vec<FixtureId>) -> Self {
        Self {
            id,
            label,
            fixtures,
        }
    }

    pub fn len(&self) -> usize {
        self.fixtures.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fixtures.is_empty()
    }
}
