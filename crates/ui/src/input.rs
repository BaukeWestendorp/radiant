use gpui::*;

pub struct IntField {
    value: i32,
}

impl IntField {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}

impl Render for IntField {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .child(self.value.to_string())
            .border_1()
            .border_color(white())
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|view, _event, _cx| {
                    view.set_value(1);
                }),
            )
    }
}
