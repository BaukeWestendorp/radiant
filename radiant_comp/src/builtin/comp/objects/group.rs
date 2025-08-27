use crate::builtin::FixtureId;

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Group {
    fids: Vec<FixtureId>,
}
