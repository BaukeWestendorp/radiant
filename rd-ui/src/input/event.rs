#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent<V> {
    Focus,
    Blur,
    Submit(V),
    Change(V),
}
