use std::f64;

use gpui::prelude::*;
use gpui::{
    App, Bounds, ClickEvent, DragMoveEvent, ElementId, EmptyView, Entity, EventEmitter,
    FocusHandle, Focusable, MouseButton, MouseUpEvent, Pixels, Point, Window, div, relative, rems,
};

use crate::container::interactive_container;
use crate::input::{FieldEvent, TextInput, TextInputEvent};
use crate::theme::ActiveTheme;
use crate::utils::{bounds_updater, z_stack};

pub struct NumberField {
    id: ElementId,
    input: Entity<TextInput>,

    min: Option<f64>,
    max: Option<f64>,
    step: Option<f64>,
    submit_on_drag: bool,

    bounds: Bounds<Pixels>,
    prev_mouse_pos: Option<Point<Pixels>>,
}

impl NumberField {
    pub fn new(
        id: impl Into<ElementId>,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let id = id.into();

        let input = cx.new(|cx| {
            let mut input = TextInput::new(id.clone(), focus_handle, window, cx)
                .px(rems(0.125).to_pixels(window.rem_size()));
            input.interactive(false);
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
                    this.input.update(cx, |input, _cx| input.interactive(false));
                    cx.emit(FieldEvent::Blur);
                }
                TextInputEvent::Submit(_) => cx.emit(FieldEvent::Submit),
                TextInputEvent::Change(_) => cx.emit(FieldEvent::Change),
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

    pub fn submit(&self, cx: &mut Context<Self>) {
        cx.emit(FieldEvent::Submit);
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
            delta / self.bounds.size.width.0 as f64
        } else {
            0.5
        }
    }
}

impl NumberField {
    fn handle_on_click(
        &mut self,
        _event: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.input.update(cx, |input, cx| {
            if !input.is_interactive() {
                input.interactive(true);
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
        let delta_x = mouse_position.x.0 - x_start.0;

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

impl Render for NumberField {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_interactive = !self.input.read(cx).is_interactive();
        let focus_handle = self.input.read(cx).focus_handle(cx);

        let slider_bar = match self.relative_value(cx) {
            Some(relative_value) => {
                div().w(relative(relative_value as f32)).h_full().bg(cx.theme().input_secondary)
            }
            None => div().size_full(),
        };

        interactive_container(ElementId::View(cx.entity_id()), Some(focus_handle))
            .flex()
            .w_full()
            .disabled(self.disabled(cx))
            .cursor_ew_resize()
            .when(!self.disabled(cx), |e| {
                e.on_click(cx.listener(Self::handle_on_click)).when(is_interactive, |e| {
                    let drag = (self.id.clone(), self.value(cx), window.mouse_position().x);
                    e.on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                        .on_drag(drag, |_, _, _, cx| cx.new(|_cx| EmptyView))
                        .on_drag_move(cx.listener(Self::handle_drag_move))
                        .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
                })
            })
            .child(
                z_stack([
                    slider_bar.into_any_element(),
                    div().py_0p5().child(self.input.clone()).into_any_element(),
                    bounds_updater(cx.entity(), |this, bounds, _cx| {
                        this.bounds = bounds;
                    })
                    .into_any_element(),
                ])
                .w_full()
                .h(window.line_height() + 2.0 * rems(0.125).to_pixels(window.rem_size())),
            )
    }
}

impl EventEmitter<FieldEvent> for NumberField {}
