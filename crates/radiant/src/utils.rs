use gpui::{App, AppContext as _, Entity};

pub fn map_entity<S: 'static, T: 'static>(
    source: Entity<S>,
    get_data: impl Fn(&Entity<S>, &mut App) -> T + 'static,
    set_data: impl Fn(&mut S, Entity<T>, &mut App) + 'static,
    cx: &mut App,
) -> Entity<T> {
    let target = cx.new(|cx| get_data(&source, cx));

    // Update the target entity when the source changes.
    cx.observe(&source, {
        let target = target.clone();
        move |source, cx| {
            let new_target_value = get_data(&source, cx);
            target.update(cx, move |target, _cx| *target = new_target_value);
        }
    })
    .detach();

    // Update the source entity when the target changes.
    cx.observe(&target, {
        let source = source.clone();
        move |to, cx| {
            source.update(cx, |source, cx| set_data(source, to, cx));
        }
    })
    .detach();

    target
}
