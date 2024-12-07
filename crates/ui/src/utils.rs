use gpui::*;

/// Stack elements on top of each other.
pub fn z_stack(children: impl IntoIterator<Item = impl IntoElement>) -> Div {
    let children = children
        .into_iter()
        .map(|child| div().size_full().child(child).absolute());
    div().relative().children(children)
}

/// Create a listener that gets the bounds of the element.
pub fn bounds_updater<V: 'static>(
    view: View<V>,
    f: impl FnOnce(&mut V, Bounds<Pixels>, &mut ViewContext<V>) + 'static,
) -> impl IntoElement {
    let view = view.clone();
    canvas(
        move |bounds, cx| view.update(cx, |view, cx| f(view, bounds, cx)),
        |_, _, _| {},
    )
    .size_full()
}
