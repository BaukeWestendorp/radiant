use rd_ui::{
    Emphasis, StyledExt, c_flex,
    comp::stateful::{Checkbox, Field, Form, FormInput, KeyPath, Picker},
    gpui::{Entity, SharedString, Window, div, prelude::*},
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
                    KeyPath::<FormData, _>::new(
                        |data| data.name.clone().into(),
                        |data, val: SharedString| data.name = val.to_string(),
                    ),
                    cx,
                ),
                cx,
            );

            form.add_input(
                FormInput::new(
                    cx.new(|cx| Picker::<Color>::from_facet("color", window, cx).unwrap()),
                    KeyPath::new(
                        |d: &FormData| d.color.clone(),
                        |d: &mut FormData, val: Option<Color>| d.color = val,
                    ),
                    cx,
                ),
                cx,
            );

            form.add_input(
                FormInput::new(
                    cx.new(|cx| Checkbox::new("tos", window, cx)),
                    KeyPath::new(|d: &FormData| d.tos, |d: &mut FormData, val: bool| d.tos = val),
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
#[derive(facet::Facet)]
#[repr(u8)]
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
