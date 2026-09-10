use rd_ui::{
    Emphasis, StyledExt, c_flex,
    comp::stateful::{Checkbox, Field, Form, FormInput, InputValue, KeyPath, Picker, PickerItem},
    gpui::{Entity, Window, div, prelude::*},
};

pub struct FormPreview {
    form: Entity<Form<FormData>>,
}

impl FormPreview {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let form = cx.new(|cx| {
            let data = cx.new(|_| FormData {
                name: String::default(),
                color: Some(Color::Red),
                tos: false,
            });
            let mut form = Form::<FormData>::new(data.clone(), window, cx);

            form.add_input(
                FormInput::new(
                    cx.new(|cx| Field::new("name", window, cx)),
                    KeyPath::new(
                        |d: &FormData| InputValue::Valid(d.name.clone()),
                        |d: &mut FormData, val: String| d.name = val,
                    ),
                    cx,
                ),
                cx,
            );

            form.add_input(
                FormInput::new(
                    cx.new(|cx| {
                        Picker::<Color>::new(
                            "color",
                            vec![
                                PickerItem::new("red", Color::Red),
                                PickerItem::new("green", Color::Green),
                                PickerItem::new("blue", Color::Blue),
                            ],
                            window,
                            cx,
                        )
                    }),
                    KeyPath::new(
                        |d: &FormData| InputValue::Valid(d.color.clone()),
                        |d: &mut FormData, val: Option<Color>| d.color = val,
                    ),
                    cx,
                ),
                cx,
            );

            form.add_input(
                FormInput::new(
                    cx.new(|cx| Checkbox::new("tos", window, cx)),
                    KeyPath::new(
                        |d: &FormData| InputValue::Valid(d.tos),
                        |d: &mut FormData, val: bool| d.tos = val,
                    ),
                    cx,
                ),
                cx,
            );

            form
        });

        Self { form }
    }
}

impl Render for FormPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        c_flex()
            .size_full()
            .gap_4()
            .child(
                div()
                    .w_96()
                    .p_2()
                    .emphasis_bordered(Emphasis::Primary, cx)
                    .child(self.form.clone()),
            )
            .child(format!("{:?}", self.form.read(cx).data().read(cx)))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone)]
pub struct FormData {
    pub name: String,
    pub color: Option<Color>,
    pub tos: bool,
}
