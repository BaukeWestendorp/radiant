use super::{TextInput, TextInputEvent};
use crate::{Disableable, InteractiveContainer};
use gpui::*;
use prelude::FluentBuilder;

pub struct NumberField {
    input: Entity<TextInput>,

    prev_mouse_pos: Option<Point<Pixels>>,
}

impl NumberField {
    pub fn new(id: impl Into<ElementId>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            let mut input = TextInput::new(id, window, cx).p(window.rem_size() * 0.25);
            input.set_is_interactive(false);
            input
        });

        cx.subscribe(&input, |_number_field, input, event, cx| match event {
            TextInputEvent::Blur => input.update(cx, |input, _cx| input.set_is_interactive(false)),
            _ => {}
        })
        .detach();

        Self { input, prev_mouse_pos: None }
    }

    pub fn disabled(&self, cx: &App) -> bool {
        self.input.read(cx).disabled()
    }

    pub fn set_disabled(&self, disabled: bool, cx: &mut App) {
        self.input.update(cx, |text_field, _cx| text_field.set_disabled(disabled));
    }

    pub fn masked(&self, cx: &App) -> bool {
        self.input.read(cx).masked()
    }

    pub fn set_masked(&self, masked: bool, cx: &mut App) {
        self.input.update(cx, |text_field, _cx| text_field.set_masked(masked));
    }

    pub fn value(&self, cx: &App) -> f64 {
        let value_str = self.input.read(cx).text();
        value_str.parse().expect("should always be able to parse value string")
    }

    pub fn set_value(&self, value: f64, cx: &mut App) {
        self.input.update(cx, |text_field, cx| {
            let value_str = value.to_string().into();
            text_field.set_text(value_str, cx);
        })
    }
}

impl NumberField {
    fn handle_on_click(
        &mut self,
        _event: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.input.update(cx, |input, _cx| input.set_is_interactive(true));
    }

    fn handle_drag_move(
        &mut self,
        _event: &DragMoveEvent<()>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mouse_position = window.mouse_position();
        let diff = self.prev_mouse_pos.map_or(Point::default(), |prev| mouse_position - prev);

        let factor = 0.5;
        self.set_value(self.value(cx) + diff.x.to_f64() * factor, cx);

        self.prev_mouse_pos = Some(mouse_position);
    }

    fn handle_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _cx: &mut Context<Self>) {
        self.prev_mouse_pos = None;
    }
}

impl Render for NumberField {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_interactive = !self.input.read(cx).is_interactive();
        let focus_handle = self.input.read(cx).focus_handle(cx);

        InteractiveContainer::new(ElementId::View(cx.entity_id()), focus_handle)
            .disabled(self.disabled(cx))
            .when(!self.disabled(cx), |e| {
                e.on_click(cx.listener(Self::handle_on_click)).when(is_interactive, |e| {
                    e.cursor_ew_resize()
                        .on_drag((), |_, _, _, cx| cx.new(|_cx| EmptyView))
                        .on_drag_move(cx.listener(Self::handle_drag_move))
                        .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
                })
            })
            .child(self.input.clone())
    }
}
