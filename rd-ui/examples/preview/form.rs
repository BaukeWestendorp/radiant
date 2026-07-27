use gpui::{App, Entity, Window, div, prelude::*};
use rd_ui::{Field, Form, FormDelegate, FormField, Input, InputEvent, InputState, Picker, Slider};

pub struct FormPreview {
    form: Entity<InputState<Form<PreviewForm>>>,
}

impl FormPreview {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let form = cx.new(|cx| {
            InputState::new(
                Form::new(PreviewForm::new(window, cx), cx.focus_handle(), window, cx),
                window,
                cx,
            )
        });

        cx.subscribe(&form, |_, _, event, _| match event {
            InputEvent::Submit(data) => {
                dbg!(data);
            }
            _ => {}
        })
        .detach();

        Self { form }
    }
}

impl Render for FormPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p_2().child(Input::new(self.form.clone()))
    }
}

struct PreviewForm {
    enum_value: Entity<InputState<Picker<EnumValue>>>,
    name: Entity<InputState<Field<String>>>,
    slider: Entity<InputState<Slider<f64>>>,
    picker: Entity<InputState<Picker<EnumValue>>>,
}

impl PreviewForm {
    fn new(window: &mut Window, cx: &mut Context<InputState<Form<Self>>>) -> Self {
        Self {
            enum_value: cx.new(|cx| {
                InputState::new(
                    Picker::dropdown(EnumValue::Alpha, cx.focus_handle(), window, cx),
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
            picker: cx.new(|cx| {
                InputState::new(
                    Picker::inline(EnumValue::Alpha, cx.focus_handle(), window, cx),
                    window,
                    cx,
                )
            }),
        }
    }
}

impl FormDelegate for PreviewForm {
    type Data = PreviewFormData;

    fn fields(&self, cx: &mut App) -> Vec<FormField> {
        vec![
            FormField::new("Enum Value", Input::new(self.enum_value.clone()), cx),
            FormField::new("Name", Input::new(self.name.clone()), cx),
            FormField::new("Slider", Input::new(self.slider.clone()), cx),
            FormField::new("Picker", Input::new(self.picker.clone()), cx),
        ]
    }

    fn extract_data(&self, cx: &App) -> Option<Self::Data> {
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

#[derive(Debug, Clone, PartialEq)]
#[derive(rd_ui::Input)]
enum EnumValue {
    Alpha,
    Beta,
    Gamma,
}
