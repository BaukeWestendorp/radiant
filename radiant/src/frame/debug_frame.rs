use gpui::*;
use ui::{NumberField, TextFieldEvent};

pub struct DebugFrame {
    number_field: Entity<NumberField>,
}

impl DebugFrame {
    pub fn build(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let number_field = cx.new(|cx| {
                let field = NumberField::new(window, cx);
                field.set_value(42.7, cx);
                field.set_disabled(true, cx);
                field
            });

            cx.subscribe(
                &number_field,
                |_this, _text_field, event: &TextFieldEvent, _cx: &mut Context<Self>| {
                    dbg!(&event);
                },
            )
            .detach();

            Self { number_field }
        })
    }
}

impl Render for DebugFrame {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p_2().child(self.number_field.clone())
    }
}
