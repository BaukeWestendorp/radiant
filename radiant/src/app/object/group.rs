use zeevonk::project::stage::FixtureId;

pub type GroupId = u32;

pub struct Group {
    pub name: String,
    pub fixture_ids: Vec<FixtureId>,
}
