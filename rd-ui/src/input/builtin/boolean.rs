use gpui::{App, Entity, Window, prelude::*};

use crate::{AutoInput, Dropdown, DropdownValue, InputState};

impl DropdownValue for bool {
    fn variants() -> Vec<Self> {
        vec![false, true]
    }

    fn label(&self) -> String {
        match self {
            false => "No".to_string(),
            true => "Yes".to_string(),
        }
    }
}

impl AutoInput for bool {
    type Delegate = Dropdown<bool>;

    fn build_input(
        initial_value: Self,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<InputState<Self::Delegate>> {
        cx.new(move |cx| {
            let dropdown = Dropdown::new(initial_value, cx.focus_handle(), window, cx);
            InputState::new(dropdown, window, cx)
        })
    }
}
