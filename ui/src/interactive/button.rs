use gpui::prelude::*;
use gpui::{ElementId, FocusHandle, div};

use crate::{InteractiveContainer, interactive_container};

pub fn button(
    id: impl Into<ElementId>,
    focus_handle: Option<FocusHandle>,
    content: impl IntoElement,
) -> InteractiveContainer {
    interactive_container(id, focus_handle)
        .cursor_pointer()
        .child(div().px_2().py_1().child(content))
}
