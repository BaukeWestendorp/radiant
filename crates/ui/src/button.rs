use gpui::*;

use crate::{theme::ActiveTheme, InteractiveContainer};

// FIXME: Convert this to a struct.
pub fn button(
    label: SharedString,
    id: impl Into<ElementId>,
    cx: &AppContext,
) -> InteractiveContainer {
    InteractiveContainer::new(id.into(), false, false)
        .h(cx.theme().input_height)
        .flex()
        .items_center()
        .min_w_10()
        .px_1()
        .child(label)
}
