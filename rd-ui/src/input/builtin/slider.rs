use gpui::{
    App, Bounds, ClickEvent, DragMoveEvent, ElementId, Entity, FocusHandle, Focusable, MouseButton,
    MouseUpEvent, Pixels, Point, RenderOnce, Window, canvas, div, prelude::*, relative, rems,
};

use crate::{
    ActiveTheme, InputDelegate, InputEvent, InputState, TextInput, interactive_container, z_stack,
};

pub struct Slider {
    element_id: ElementId,
    text_input: Entity<TextInput>,

    min: Option<f64>,
    max: Option<f64>,
    step: Option<f64>,
    submit_on_drag: bool,

    bounds: Bounds<Pixels>,
    prev_mouse_pos: Option<Point<Pixels>>,
}

impl Slider {
    pub fn new(
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<InputState<Self>>,
    ) -> Self {
        let element_id = ElementId::View(cx.entity_id());

        let text_input = cx.new(|cx| {
            let mut text_input = TextInput::new(element_id.clone(), focus_handle, window, cx)
                .px(rems(0.125).to_pixels(window.rem_size()));
            text_input.set_text("0".into(), cx);
            text_input.set_interactive(false, cx);
            text_input.set_validator(|text| {
                text.trim().is_empty()
                    || regex::Regex::new(r"^[+-]?(\d+\.?\d*|\.\d+)?$")
                        .unwrap()
                        .is_match(text.trim())
            });
            text_input
        });

        cx.subscribe(&text_input, |this, _, event, cx| {
            cx.notify();
            match event {
                InputEvent::Focus => cx.emit(InputEvent::Focus),
                InputEvent::Blur => {
                    this.commit_value(cx);
                    this.text_input.update(cx, |input, cx| input.set_interactive(false, cx));
                    cx.emit(InputEvent::Blur);
                }
                InputEvent::Submit(s) => {
                    if let Ok(v) = s.parse() {
                        cx.emit(InputEvent::Submit(v))
                    }
                }
                InputEvent::Change(s) => {
                    if let Ok(v) = s.parse() {
                        cx.emit(InputEvent::Change(v))
                    }
                }
            }
        })
        .detach();

        Self {
            element_id,
            text_input,

            min: None,
            max: None,
            step: None,
            submit_on_drag: true,

            bounds: Bounds::default(),
            prev_mouse_pos: None,
        }
    }

    pub fn min(&self) -> Option<f64> {
        self.min
    }

    pub fn set_min(&mut self, min: Option<f64>, cx: &mut Context<InputState<Self>>) {
        self.min = min;
        self.commit_value(cx);
    }

    pub fn with_min(mut self, min: Option<f64>, cx: &mut Context<InputState<Self>>) -> Self {
        self.set_min(min, cx);
        self
    }

    pub fn max(&self) -> Option<f64> {
        self.max
    }

    pub fn set_max(&mut self, max: Option<f64>, cx: &mut Context<InputState<Self>>) {
        self.max = max;
        self.commit_value(cx);
    }

    pub fn with_max(mut self, max: Option<f64>, cx: &mut Context<InputState<Self>>) -> Self {
        self.set_max(max, cx);
        self
    }

    pub fn step(&self) -> Option<f64> {
        self.step
    }

    pub fn set_step(&mut self, step: Option<f64>, cx: &mut Context<InputState<Self>>) {
        self.step = step;
        self.commit_value(cx);
    }

    pub fn with_step(mut self, step: Option<f64>, cx: &mut Context<InputState<Self>>) -> Self {
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
        self.text_input.read(cx).disabled()
    }

    pub fn set_disabled(&self, disabled: bool, cx: &mut App) {
        self.text_input.update(cx, |text_input, _cx| text_input.set_disabled(disabled));
    }

    pub fn with_disabled(self, disabled: bool, cx: &mut App) -> Self {
        self.set_disabled(disabled, cx);
        self
    }

    pub fn masked(&self, cx: &App) -> bool {
        self.text_input.read(cx).masked()
    }

    pub fn set_masked(&self, masked: bool, cx: &mut App) {
        self.text_input.update(cx, |text_input, _cx| text_input.set_masked(masked));
    }

    pub fn with_masked(self, masked: bool, cx: &mut App) -> Self {
        self.set_masked(masked, cx);
        self
    }

    pub fn value(&self, cx: &App) -> Option<f64> {
        let value_str = self.text_input.read(cx).text().to_string();
        if value_str.trim().is_empty() {
            return None;
        };
        Some(value_str.parse().unwrap_or_default())
    }

    pub fn set_value(&mut self, value: Option<f64>, cx: &mut App) {
        let Some(value) = value else {
            self.text_input.update(cx, |text_input, cx| {
                text_input.set_text("".into(), cx);
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

        self.text_input.update(cx, |text_input, cx| {
            let value_str = value.to_string().into();
            text_input.set_text(value_str, cx);
        })
    }

    pub fn with_value(mut self, value: Option<f64>, cx: &mut App) -> Self {
        self.set_value(value, cx);
        self
    }

    pub fn set_validator<F: Fn(&str) -> bool + 'static>(&self, cx: &mut App, validator: F) {
        self.text_input.update(cx, |text_input, _cx| text_input.set_validator(validator));
    }

    pub fn with_validator<F: Fn(&str) -> bool + 'static>(self, cx: &mut App, validator: F) -> Self {
        self.set_validator(cx, validator);
        self
    }

    pub fn set_submit_validator<F: Fn(&str) -> bool + 'static>(&self, cx: &mut App, validator: F) {
        self.text_input.update(cx, |text_input, _cx| text_input.set_submit_validator(validator));
    }

    pub fn with_submit_validator<F: Fn(&str) -> bool + 'static>(
        self,
        cx: &mut App,
        validator: F,
    ) -> Self {
        self.set_submit_validator(cx, validator);
        self
    }

    pub fn submit(&self, cx: &mut Context<InputState<Self>>) {
        if let Some(v) = self.value(cx) {
            cx.emit(InputEvent::Submit(v));
        }
    }

    fn commit_value(&mut self, cx: &mut Context<InputState<Self>>) {
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
        cx: &mut Context<InputState<Self>>,
    ) {
        self.text_input.update(cx, |input, cx| {
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
        cx: &mut Context<InputState<Self>>,
    ) {
        let (id, start_value, x_start) = event.drag(cx);

        if &self.element_id != id {
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

    fn handle_mouse_up(
        &mut self,
        _: &MouseUpEvent,
        _window: &mut Window,
        _cx: &mut Context<InputState<Self>>,
    ) {
        self.prev_mouse_pos = None;
    }
}

impl InputDelegate for Slider {
    type Value = f64;

    fn new_element(
        state: Entity<InputState<Self>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> impl IntoElement {
        SliderElement { state }
    }
}

impl Focusable for Slider {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.text_input.focus_handle(cx)
    }
}

#[derive(IntoElement)]
struct SliderElement {
    state: Entity<InputState<Slider>>,
}

impl RenderOnce for SliderElement {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.state.read(cx);
        let is_interactive = !state.text_input.read(cx).is_interactive();
        let is_at_either_end = state.relative_value(cx).map_or(false, |v| v <= 0.0 || v >= 1.0);
        let focus_handle = state.text_input.read(cx).focus_handle(cx);
        let disabled = state.disabled(cx);

        let relative_value = state.relative_value(cx);

        let slider_bar = match relative_value {
            Some(relative_value) => div()
                .w(relative(relative_value as f32))
                .h_full()
                .bg(cx.theme().bg_tertiary)
                .when(!is_at_either_end, |e| e.border_r_1())
                .border_color(cx.theme().border_primary),
            None => div().size_full(),
        };

        let element_id = state.element_id.clone();
        let value = state.value(cx);
        let text_input = state.text_input.clone();
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
                    let drag = (element_id, value, window.mouse_position().x);
                    e.on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                        .on_drag(drag, |_, _, _, cx| cx.new(|_cx| gpui::EmptyView))
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
                    div().py_0p5().child(text_input).into_any_element(),
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
