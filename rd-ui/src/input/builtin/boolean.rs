use gpui::{App, Entity, Window, prelude::*};

use crate::{AutoInput, InputState, Picker};

impl AutoInput for bool {
    type Delegate = Picker<bool>;

    fn build_input(
        initial_value: Self,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<InputState<Self::Delegate>> {
        cx.new(move |cx| {
            let picker = Picker::builder(initial_value, [true, false])
                .label_fn(|v| match v {
                    true => "Yes".to_string(),
                    false => "No".to_string(),
                })
                .build(cx.focus_handle(), window, cx);
            InputState::new(picker, window, cx)
        })
    }
}
