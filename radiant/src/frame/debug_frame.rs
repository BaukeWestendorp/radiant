use gpui::*;
use ui::{NumberField, TextField};

pub struct DebugFrame {
    text_field: Entity<TextField>,
    number_field: Entity<NumberField>,
}

impl DebugFrame {
    pub fn build(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let text_field = cx.new(|cx| {
                let field = TextField::new("text_field_1", window, cx);
                field.set_value("Text Field Value".into(), cx);
                field
            });

            let number_field = cx.new(|cx| {
                let mut field = NumberField::new("number_field_1", window, cx);
                field.set_value(0.2, cx);
                field.set_min(Some(0.0));
                field.set_max(Some(1.0));
                field
            });

            Self { text_field, number_field }
        })
    }
}

impl Render for DebugFrame {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_2()
            .p_2()
            .child(self.number_field.clone())
            .child(self.text_field.clone())
    }
}
