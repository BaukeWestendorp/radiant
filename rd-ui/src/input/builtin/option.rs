use gpui::{App, Entity, Window, prelude::*};

use crate::{
    AutoInput, Dropdown, DropdownValue, Form, FormDelegate, FormField, Input, InputDelegate,
    InputState,
};

#[derive(Clone, PartialEq)]
enum HasValue {
    No,
    Yes,
}

impl DropdownValue for HasValue {
    fn variants() -> Vec<Self> {
        vec![HasValue::No, HasValue::Yes]
    }

    fn label(&self) -> String {
        match self {
            HasValue::No => "No".to_string(),
            HasValue::Yes => "Yes".to_string(),
        }
    }
}

impl AutoInput for HasValue {
    type Delegate = Dropdown<HasValue>;

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

pub struct OptionInput<T: AutoInput> {
    toggle: Entity<InputState<Dropdown<HasValue>>>,
    value: Entity<InputState<T::Delegate>>,
}

impl<T: AutoInput + Default + Clone> FormDelegate for OptionInput<T> {
    type Data = Option<T>;

    fn fields(&self, cx: &mut App) -> Vec<FormField> {
        let mut fields = vec![FormField::new("Has Value", Input::new(self.toggle.clone()), cx)];

        let toggle_val = self.toggle.read(cx).value(cx).clone();
        if matches!(toggle_val, HasValue::Yes) {
            fields.push(FormField::new("Value", Input::new(self.value.clone()), cx));
        }

        fields
    }

    fn extract_data(&self, cx: &App) -> Option<Self::Data> {
        let toggle_val = self.toggle.read(cx).value(cx).clone();

        match toggle_val {
            HasValue::No => Some(None),
            HasValue::Yes => {
                let inner_val = self.value.read(cx).delegate().value_or_default(cx).clone();
                Some(Some(inner_val))
            }
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
        use gpui::AppContext as _;

        let init_toggle = if initial_value.is_some() { HasValue::Yes } else { HasValue::No };
        let toggle_entity = <HasValue as AutoInput>::build_input(init_toggle, window, cx);

        let init_val = initial_value.unwrap_or_default();
        let value_entity = <T as AutoInput>::build_input(init_val, window, cx);

        cx.new(move |cx| {
            let delegate = OptionInput { toggle: toggle_entity, value: value_entity };

            let form = Form::new(delegate, cx.focus_handle(), window, cx);
            InputState::new(form, window, cx)
        })
    }
}
