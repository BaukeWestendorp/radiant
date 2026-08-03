use gpui::{App, Entity, Focusable, Window, prelude::*};

use crate::{AutoInput, Form, FormDelegate, FormField, Input, InputDelegate, InputState, Picker};

pub struct OptionInput<T: AutoInput> {
    has_value: Entity<InputState<Picker<bool>>>,
    value: Entity<InputState<T::Delegate>>,
}

impl<T: AutoInput + Default + Clone> FormDelegate for OptionInput<T> {
    type Data = Option<T>;

    fn fields(&self, cx: &mut App) -> Vec<FormField> {
        let mut fields =
            vec![FormField::new(Input::new(self.has_value.clone()), cx).with_label("Has Value")];

        let has_value = self.has_value.read(cx).value(cx).clone();
        if has_value {
            fields.push(FormField::new(Input::new(self.value.clone()), cx).with_label("Value"));
        }

        fields
    }

    fn extract_data(&self, cx: &App) -> Option<Self::Data> {
        let has_value = self.has_value.read(cx).value(cx).clone();
        match has_value {
            false => Some(None),
            true => {
                let inner_val = self.value.read(cx).delegate().value_or_default(cx).clone();
                Some(Some(inner_val))
            }
        }
    }

    fn preferred_focus_handle(&self, cx: &App) -> Option<gpui::FocusHandle> {
        if self.has_value.read(cx).value(cx).clone() {
            Some(self.value.focus_handle(cx))
        } else {
            Some(self.has_value.focus_handle(cx))
        }
    }
}

impl<T: AutoInput + Default + Clone> AutoInput for Option<T> {
    type Delegate = Form<OptionInput<T>>;

    fn build_input(
        initial_value: Self,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<InputState<Self::Delegate>> {
        let as_value = <bool as AutoInput>::build_input(initial_value.is_some(), window, cx);

        let initial_value = initial_value.unwrap_or_default();
        let value_entity = <T as AutoInput>::build_input(initial_value, window, cx);

        cx.new(move |cx| {
            let delegate = OptionInput { has_value: as_value, value: value_entity };
            let form = Form::new(delegate, cx.focus_handle(), window, cx);
            InputState::new(form, window, cx)
        })
    }
}
