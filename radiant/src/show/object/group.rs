use crate::show::FixtureId;

#[derive(object_derive::Object)]
#[object_derive::object]
#[derive(Clone, Default)]
#[derive(serde::Deserialize)]
pub struct Group {
    pub(crate) fids: Vec<FixtureId>,
}

impl Group {
    pub fn fids(&self) -> &[FixtureId] {
        &self.fids
    }
}
