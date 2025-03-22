use super::{TextInput, TextInputEvent};
use crate::{Disableable, InteractiveContainer};
use gpui::*;
use prelude::FluentBuilder;

pub struct NumberField {
    input: Entity<TextInput>,

    step: Option<f64>,
    min: Option<f64>,
    max: Option<f64>,

    prev_mouse_pos: Option<Point<Pixels>>,
}

impl NumberField {
    pub fn new(id: impl Into<ElementId>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            let mut input = TextInput::new(id, window, cx).p(window.rem_size() * 0.25);
            input.set_is_interactive(false);
            input
        });

        cx.subscribe(&input, |number_field, input, event, cx| match event {
            TextInputEvent::Blur => {
                number_field.commit_value(cx);
                input.update(cx, |input, _cx| input.set_is_interactive(false));
            }
            _ => {}
        })
        .detach();

        Self { input, step: None, min: None, max: None, prev_mouse_pos: None }
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
        value_str.parse().unwrap_or_default()
    }

    pub fn set_value(&self, value: f64, cx: &mut App) {
        let min = self.min().unwrap_or(f64::MIN);
        let max = self.max().unwrap_or(f64::MAX);
        let mut value = value.clamp(min, max);

        if let Some(step) = self.step() {
            value = (value / step).round() * step;
        }

        self.input.update(cx, |text_field, cx| {
            let value_str = value.to_string().into();
            text_field.set_text(value_str, cx);
        })
    }

    fn commit_value(&self, cx: &mut App) {
        self.set_value(self.value(cx), cx);
    }

    pub fn step(&self) -> Option<f64> {
        self.step
    }

    pub fn set_step(&mut self, step: Option<f64>) {
        self.step = step;
    }

    pub fn min(&self) -> Option<f64> {
        self.min
    }

    pub fn set_min(&mut self, min: Option<f64>) {
        self.min = min;
    }

    pub fn max(&self) -> Option<f64> {
        self.max
    }

    pub fn set_max(&mut self, max: Option<f64>) {
        self.max = max;
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
