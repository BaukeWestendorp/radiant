use gpui::{App, Entity, Window, prelude::*};

use crate::{AutoInput, InputState, Picker, PickerValue};

impl PickerValue for bool {
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
    type Delegate = Picker<bool>;

    fn build_input(
        initial_value: Self,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<InputState<Self::Delegate>> {
        cx.new(move |cx| {
            let picker = Picker::inline(initial_value, cx.focus_handle(), window, cx);
            InputState::new(picker, window, cx)
        })
    }
}
