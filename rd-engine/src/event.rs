use zeevonk::project::FixtureId;

#[derive(Clone, Debug)]
pub enum Event {
    SelectionChanged(Vec<FixtureId>),
    HighlightChanged(bool),
}
