use gpui::prelude::*;
use gpui::{Entity, Window, div};
use rd_ui::{Dropdown, Form, FormDelegate, FormEvent, FormField, FormState, InputState};

pub struct FormPreview {
    form: Entity<FormState<PreviewForm>>,
}

impl FormPreview {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let form = cx.new(|cx| FormState::new(PreviewForm::new(window, cx), window, cx));

        cx.subscribe(&form, |_, _, event, _| match event {
            FormEvent::Submit { data } => {
                dbg!(data);
            }
        })
        .detach();

        Self { form }
    }
}

impl Render for FormPreview {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p_2().child(Form::new(self.form.clone(), window, cx))
    }
}

struct PreviewForm {
    enum_value: Entity<InputState<Dropdown<EnumValue>>>,
}

impl PreviewForm {
    fn new(window: &mut Window, cx: &mut Context<FormState<Self>>) -> Self {
        Self {
            enum_value: cx.new(|cx| {
                InputState::new(
                    Dropdown::new(EnumValue::Alpha, cx.focus_handle(), window, cx),
                    window,
                    cx,
                )
            }),
        }
    }
}

impl FormDelegate for PreviewForm {
    type Data = PreviewFormData;

    fn fields(&self) -> Vec<FormField> {
        vec![FormField::new("Enum Value", rd_ui::Input::new(self.enum_value.clone()))]
    }

    fn extract_data(&self, cx: &gpui::App) -> Option<Self::Data> {
        let input_state = self.enum_value.read(cx);

        let current_enum_value = input_state.value().read(cx).clone();

        Some(PreviewFormData { enum_value: current_enum_value })
    }
}

#[derive(Debug)]
struct PreviewFormData {
    enum_value: EnumValue,
}

#[derive(Debug, Clone)]
#[derive(rd_ui::Input)]
enum EnumValue {
    Alpha,
    Beta,
    Gamma,
}
