use gpui::*;
use ui::TextField;

pub struct DebugFrame {
    text_field: Entity<TextField>,
}

impl DebugFrame {
    pub fn build(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let text_field = cx.new(|cx| TextField::new(window, cx));
            Self { text_field }
        })
    }
}

impl Render for DebugFrame {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p_2().child(self.text_field.clone())
    }
}
