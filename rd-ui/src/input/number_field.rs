use std::f64;

use gpui::{
    App, Bounds, ClickEvent, DragMoveEvent, ElementId, EmptyView, Entity, EventEmitter,
    FocusHandle, Focusable, MouseButton, MouseUpEvent, Pixels, Point, RenderOnce, Window, div,
    relative, rems,
};
use gpui::{canvas, prelude::*};

use crate::FieldEvent;
use crate::container::interactive_container;
use crate::input::text_input::{TextInput, TextInputEvent};
use crate::theme::ActiveTheme;
use crate::z_stack;

pub struct NumberFieldState {
    id: ElementId,
    input: Entity<TextInput>,

    min: Option<f64>,
    max: Option<f64>,
    step: Option<f64>,
    submit_on_drag: bool,

    bounds: Bounds<Pixels>,
    prev_mouse_pos: Option<Point<Pixels>>,
}

impl NumberFieldState {
    pub fn new(
        id: impl Into<ElementId>,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let id = id.into();

        let input = cx.new(|cx| {
            let mut input = TextInput::new(id.clone(), focus_handle.tab_stop(true), window, cx)
                .px(rems(0.125).to_pixels(window.rem_size()));
            input.set_interactive(false, cx);
            input.set_validator(|text| {
                text.trim().is_empty()
                    || regex::Regex::new(r"^[+-]?(\d+\.?\d*|\.\d+)?$")
                        .unwrap()
                        .is_match(text.trim())
            });
            input
        });

        cx.subscribe(&input, |this, _, event, cx| {
            cx.notify();
            match event {
                TextInputEvent::Focus => cx.emit(FieldEvent::Focus),
                TextInputEvent::Blur => {
                    this.commit_value(cx);
                    this.input.update(cx, |input, cx| input.set_interactive(false, cx));
                    cx.emit(FieldEvent::Blur);
                }
                TextInputEvent::Submit(s) => {
                    if let Ok(v) = s.parse() {
                        cx.emit(FieldEvent::Submit(v))
                    }
                }
                TextInputEvent::Change(s) => {
                    if let Ok(v) = s.parse() {
                        cx.emit(FieldEvent::Change(v))
                    }
                }
            }
        })
        .detach();

        Self {
            id,
            input,

            min: None,
            max: None,
            step: None,
            submit_on_drag: true,

            bounds: Bounds::default(),
            prev_mouse_pos: None,
        }
    }

    pub fn input(&self) -> &Entity<TextInput> {
        &self.input
    }

    pub fn min(&self) -> Option<f64> {
        self.min
    }

    pub fn set_min(&mut self, min: Option<f64>, cx: &mut Context<Self>) {
        self.min = min;
        self.commit_value(cx);
    }

    pub fn with_min(mut self, min: Option<f64>, cx: &mut Context<Self>) -> Self {
        self.set_min(min, cx);
        self
    }

    pub fn max(&self) -> Option<f64> {
        self.max
    }

    pub fn set_max(&mut self, max: Option<f64>, cx: &mut Context<Self>) {
        self.max = max;
        self.commit_value(cx);
    }

    pub fn with_max(mut self, max: Option<f64>, cx: &mut Context<Self>) -> Self {
        self.set_max(max, cx);
        self
    }

    pub fn step(&self) -> Option<f64> {
        self.step
    }

    pub fn set_step(&mut self, step: Option<f64>, cx: &mut Context<Self>) {
        self.step = step;
        self.commit_value(cx);
    }

    pub fn with_step(mut self, step: Option<f64>, cx: &mut Context<Self>) -> Self {
        self.set_step(step, cx);
        self
    }

    pub fn submit_on_drag(&self) -> bool {
        self.submit_on_drag
    }

    pub fn set_submit_on_drag(&mut self, submit_on_drag: bool) {
        self.submit_on_drag = submit_on_drag;
    }

    pub fn with_submit_on_drag(mut self, submit_on_drag: bool) -> Self {
        self.set_submit_on_drag(submit_on_drag);
        self
    }

    pub fn disabled(&self, cx: &App) -> bool {
        self.input.read(cx).disabled()
    }

    pub fn set_disabled(&self, disabled: bool, cx: &mut App) {
        self.input.update(cx, |text_field, _cx| text_field.set_disabled(disabled));
    }

    pub fn with_disabled(self, disabled: bool, cx: &mut App) -> Self {
        self.set_disabled(disabled, cx);
        self
    }

    pub fn masked(&self, cx: &App) -> bool {
        self.input.read(cx).masked()
    }

    pub fn set_masked(&self, masked: bool, cx: &mut App) {
        self.input.update(cx, |text_field, _cx| text_field.set_masked(masked));
    }

    pub fn with_masked(self, masked: bool, cx: &mut App) -> Self {
        self.set_masked(masked, cx);
        self
    }

    pub fn value(&self, cx: &App) -> Option<f64> {
        let value_str = self.input.read(cx).text().to_string();
        if value_str.trim().is_empty() {
            return None;
        };
        Some(value_str.parse().unwrap_or_default())
    }

    pub fn set_value(&mut self, value: Option<f64>, cx: &mut App) {
        let Some(value) = value else {
            self.input.update(cx, |text_field, cx| {
                text_field.set_text("".into(), cx);
            });
            return;
        };

        // Clamp
        let mut value = value.clamp(self.min.unwrap_or(f64::MIN), self.max.unwrap_or(f64::MAX));

        // Step
        if let Some(step) = self.step() {
            value = (value / step).round() * step;
        }

        // Round
        value = (value * 10e3f64).round() / 10e3f64;

        self.input.update(cx, |text_field, cx| {
            let value_str = value.to_string().into();
            text_field.set_text(value_str, cx);
        })
    }

    pub fn with_value(mut self, value: Option<f64>, cx: &mut App) -> Self {
        self.set_value(value, cx);
        self
    }

    pub fn set_validator<F: Fn(&str) -> bool + 'static>(&self, cx: &mut App, validator: F) {
        self.input.update(cx, |text_field, _cx| text_field.set_validator(validator));
    }

    pub fn with_validator<F: Fn(&str) -> bool + 'static>(self, cx: &mut App, validator: F) -> Self {
        self.set_validator(cx, validator);
        self
    }

    pub fn set_submit_validator<F: Fn(&str) -> bool + 'static>(&self, cx: &mut App, validator: F) {
        self.input.update(cx, |text_field, _cx| text_field.set_submit_validator(validator));
    }

    pub fn with_submit_validator<F: Fn(&str) -> bool + 'static>(
        self,
        cx: &mut App,
        validator: F,
    ) -> Self {
        self.set_submit_validator(cx, validator);
        self
    }

    pub fn submit(&self, cx: &mut Context<Self>) {
        if let Some(v) = self.value(cx) {
            cx.emit(FieldEvent::Submit(v));
        }
    }

    fn commit_value(&mut self, cx: &mut Context<Self>) {
        self.set_value(self.value(cx), cx);
        self.submit(cx);
    }

    pub fn is_slider(&self) -> bool {
        self.min().is_some() && self.max().is_some()
    }

    pub fn relative_value(&self, cx: &App) -> Option<f64> {
        if !self.is_slider() {
            return None;
        }

        let min = self.min.unwrap_or(f64::MIN);
        let max = self.max.unwrap_or(f64::MAX);
        let value = self.value(cx)?.clamp(min, max);
        Some((value - min) / (max - min))
    }

    fn drag_factor(&self) -> f64 {
        if self.is_slider() {
            let min = self.min.unwrap_or(f64::MIN);
            let max = self.max.unwrap_or(f64::MAX);
            let delta = max - min;
            delta / self.bounds.size.width.as_f32() as f64
        } else {
            0.5
        }
    }

    fn handle_on_click(
        &mut self,
        _event: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.input.update(cx, |input, cx| {
            if !input.is_interactive() {
                input.set_interactive(true, cx);
                input.select_all(cx);
            }
        });
    }

    fn handle_drag_move(
        &mut self,
        event: &DragMoveEvent<(ElementId, Option<f64>, Pixels)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let (id, start_value, x_start) = event.drag(cx);

        if &self.id != id {
            return;
        }

        let mouse_position = window.mouse_position();
        let delta_x = mouse_position.x.as_f32() - x_start.as_f32();

        let factor = self.drag_factor();
        let value = start_value.unwrap_or_default() + delta_x as f64 * factor;
        self.set_value(Some(value), cx);
        if self.submit_on_drag {
            self.commit_value(cx);
        } else {
            self.set_value(self.value(cx), cx);
        }

        self.prev_mouse_pos = Some(mouse_position);
    }

    fn handle_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _cx: &mut Context<Self>) {
        self.prev_mouse_pos = None;
    }
}

impl EventEmitter<FieldEvent<f64>> for NumberFieldState {}

#[derive(IntoElement)]
pub struct NumberField {
    state: Entity<NumberFieldState>,
}

impl NumberField {
    pub fn new(state: Entity<NumberFieldState>) -> Self {
        Self { state }
    }
}

impl Focusable for NumberField {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.state.read(cx).input.focus_handle(cx)
    }
}

impl RenderOnce for NumberField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state_ref = self.state.read(cx);
        let is_interactive = !state_ref.input.read(cx).is_interactive();
        let focus_handle = state_ref.input.read(cx).focus_handle(cx);
        let disabled = state_ref.disabled(cx);

        let relative_value = state_ref.relative_value(cx);

        let slider_bar = match relative_value {
            Some(relative_value) => {
                div().w(relative(relative_value as f32)).h_full().bg(cx.theme().bg_secondary)
            }
            None => div().size_full(),
        };

        let id = state_ref.id.clone();
        let value = state_ref.value(cx);
        let input = state_ref.input.clone();
        let entity_id = self.state.entity_id();

        let state_click = self.state.clone();
        let state_drag_move = self.state.clone();
        let state_mouse_up = self.state.clone();
        let state_canvas = self.state.clone();

        interactive_container(ElementId::View(entity_id), Some(focus_handle))
            .flex()
            .w_full()
            .disabled(disabled)
            .cursor_ew_resize()
            .when(!disabled, |e| {
                e.on_click(move |event, window, cx| {
                    state_click.update(cx, |this, cx| this.handle_on_click(event, window, cx))
                })
                .when(is_interactive, |e| {
                    let drag = (id, value, window.mouse_position().x);
                    e.on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                        .on_drag(drag, |_, _, _, cx| cx.new(|_cx| EmptyView))
                        .on_drag_move(move |event, window, cx| {
                            state_drag_move
                                .update(cx, |this, cx| this.handle_drag_move(event, window, cx))
                        })
                        .on_mouse_up(MouseButton::Left, move |event, window, cx| {
                            state_mouse_up
                                .update(cx, |this, cx| this.handle_mouse_up(event, window, cx))
                        })
                })
            })
            .child(
                z_stack([
                    slider_bar.into_any_element(),
                    div().py_0p5().child(input).into_any_element(),
                    canvas(
                        move |bounds, _, cx| {
                            state_canvas.update(cx, |this, cx| {
                                this.bounds = bounds;
                                cx.notify();
                            });
                        },
                        |_, _, _, _| {},
                    )
                    .into_any_element(),
                ])
                .w_full()
                .h(window.line_height() + 2.0 * rems(0.125).to_pixels(window.rem_size())),
            )
    }
}
