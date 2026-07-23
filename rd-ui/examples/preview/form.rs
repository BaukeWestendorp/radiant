use gpui::{Entity, Window, div, prelude::*};
use rd_ui::{
    Dropdown, Field, Form, FormDelegate, FormEvent, FormField, FormState, Input, InputState, Slider,
};

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
    name: Entity<InputState<Field<String>>>,
    slider: Entity<InputState<Slider<f64>>>,
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
            name: cx
                .new(|cx| InputState::new(Field::new(cx.focus_handle(), window, cx), window, cx)),
            slider: cx.new(|cx| {
                InputState::new(
                    Slider::new(cx.focus_handle(), window, cx)
                        .with_min(Some(0.0), cx)
                        .with_max(Some(1.0), cx),
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
        vec![
            FormField::new("Enum Value", Input::new(self.enum_value.clone())),
            FormField::new("Name", Input::new(self.name.clone())),
            FormField::new("Slider", Input::new(self.slider.clone())),
        ]
    }

    fn extract_data(&self, cx: &gpui::App) -> Option<Self::Data> {
        Some(PreviewFormData {
            enum_value: self.enum_value.read(cx).value(cx).clone(),
            name: self.name.read(cx).value(cx)?.clone(),
            slider: self.slider.read(cx).value(cx)?.clone(),
        })
    }
}

#[allow(unused)]
#[derive(Debug)]
struct PreviewFormData {
    enum_value: EnumValue,
    name: String,
    slider: f64,
}

#[derive(Debug, Clone)]
#[derive(rd_ui::Input)]
enum EnumValue {
    Alpha,
    Beta,
    Gamma,
}
