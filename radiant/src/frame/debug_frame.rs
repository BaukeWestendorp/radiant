use gpui::*;
use ui::{TextField, TextFieldEvent};

pub struct DebugFrame {
    text_field: Entity<TextField>,
}

impl DebugFrame {
    pub fn build(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let text_field = cx.new(|cx| {
                let mut text_field = TextField::new(window, cx);
                text_field.set_text("hidden".into(), cx);
                text_field
            });

            cx.subscribe(
                &text_field,
                |_this, _text_field, event: &TextFieldEvent, _cx: &mut Context<Self>| {
                    dbg!(&event);
                },
            )
            .detach();

            Self { text_field }
        })
    }
}

impl Render for DebugFrame {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p_2().child(self.text_field.clone())
    }
}
