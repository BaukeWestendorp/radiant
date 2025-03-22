use gpui::*;
use prelude::FluentBuilder;

use crate::theme::ActiveTheme;

use super::{TextInput, TextInputEvent};

pub struct NumberField {
    input: Entity<TextInput>,

    prev_mouse_pos: Option<Point<Pixels>>,
}

impl NumberField {
    pub fn new(id: impl Into<ElementId>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            let mut input = TextInput::new(id, window, cx).p(window.rem_size() * 0.25);
            input.set_interactive(false, cx);
            input
        });

        cx.subscribe(&input, |_number_field, input, event, cx| match event {
            TextInputEvent::Blur => input.update(cx, |input, cx| input.set_interactive(false, cx)),
            _ => {}
        })
        .detach();

        Self { input, prev_mouse_pos: None }
    }

    pub fn disabled(&self, cx: &App) -> bool {
        self.input.read(cx).disabled()
    }

    pub fn set_disabled(&self, disabled: bool, cx: &mut App) {
        self.input.update(cx, |text_field, cx| text_field.set_disabled(disabled, cx));
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
        self.input.update(cx, |input, cx| input.set_interactive(true, cx));
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let interactive = !self.input.read(cx).interactive();
        let focused = self.input.read(cx).is_focused(window);

        let background_color =
            if focused { cx.theme().background_focused } else { cx.theme().background };

        let border_color = if focused {
            cx.theme().border_color_focused
        } else if self.disabled(cx) {
            cx.theme().border_color_muted
        } else {
            cx.theme().border_color
        };

        let text_color =
            if self.disabled(cx) { cx.theme().text_muted } else { cx.theme().text_primary };

        div()
            .id("number_field")
            .w_full()
            .bg(background_color)
            .text_color(text_color)
            .border_1()
            .border_color(border_color)
            .rounded(cx.theme().radius)
            .on_click(cx.listener(Self::handle_on_click))
            .cursor_ew_resize()
            .when(interactive, |e| {
                e.on_drag((), |_, _, _, cx| cx.new(|_cx| EmptyView))
                    .on_drag_move(cx.listener(Self::handle_drag_move))
                    .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            })
            .child(self.input.clone())
    }
}
