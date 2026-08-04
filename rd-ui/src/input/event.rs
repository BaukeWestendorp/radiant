#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent<V> {
    Submit(V),
    Change(V),
}
