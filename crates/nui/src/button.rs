use gpui::prelude::*;
use gpui::{ElementId, FocusHandle, div};

use crate::container::{InteractiveContainer, interactive_container};

pub fn button(
    id: impl Into<ElementId>,
    focus_handle: Option<FocusHandle>,
    content: impl IntoElement,
) -> InteractiveContainer {
    interactive_container(id, focus_handle)
        .child(div().flex().justify_center().items_center().px_2().child(content))
}
