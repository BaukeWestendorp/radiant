pub trait Selectable {
    fn selected(self, selected: bool) -> Self;
}
