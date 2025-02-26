use gpui::*;

/// Stack elements on top of each other.
pub fn z_stack(children: impl IntoIterator<Item = impl IntoElement>) -> Div {
    let children = children.into_iter().map(|child| div().size_full().child(child).absolute());
    div().relative().children(children)
}

/// Create a listener that gets the bounds of the element.
pub fn bounds_updater<V: 'static>(
    entity: Entity<V>,
    f: impl FnOnce(&mut V, Bounds<Pixels>, &mut Context<V>) + 'static,
) -> impl IntoElement {
    let entity = entity.clone();
    canvas(
        move |bounds, _window, cx| entity.update(cx, |view, cx| f(view, bounds, cx)),
        |_, _, _, _| {},
    )
    .size_full()
}
