use super::{TextInput, TextInputEvent};
use crate::{Disableable, InteractiveContainer, bounds_updater, theme::ActiveTheme, z_stack};
use gpui::*;
use prelude::FluentBuilder;

pub struct NumberField {
    id: ElementId,
    input: Entity<TextInput>,

    step: Option<f64>,
    min: Option<f64>,
    max: Option<f64>,

    bounds: Bounds<Pixels>,

    prev_mouse_pos: Option<Point<Pixels>>,
}

impl NumberField {
    pub fn new(id: impl Into<ElementId>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let id = id.into();

        let input = cx.new(|cx| {
            let mut input = TextInput::new(id.clone(), window, cx).px(window.rem_size() * 0.25);
            input.set_is_interactive(false);
            input
        });

        cx.subscribe(&input, |number_field, input, event, cx| {
            cx.emit(event.clone());
            cx.notify();
            match event {
                TextInputEvent::Blur => {
                    number_field.commit_value(cx);
                    input.update(cx, |input, _cx| input.set_is_interactive(false));
                }
                _ => {}
            }
        })
        .detach();

        Self {
            id,
            input,
            step: None,
            min: None,
            max: None,
            bounds: Bounds::default(),
            prev_mouse_pos: None,
        }
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
        // Clamp
        let min = self.min().unwrap_or(f64::MIN);
        let max = self.max().unwrap_or(f64::MAX);
        let mut value = value.clamp(min, max);

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

    pub fn is_slider(&self) -> bool {
        self.min.is_some() && self.max.is_some()
    }

    pub fn relative_value(&self, cx: &App) -> Option<f64> {
        let min = self.min()?;
        let max = self.max()?;
        let value = self.value(cx).clamp(min, max);
        Some((value - min) / (max - min))
    }

    fn drag_factor(&self) -> f64 {
        if self.is_slider() {
            let delta = self.max.unwrap() - self.min.unwrap();
            delta / self.bounds.size.width.to_f64()
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
        self.input.update(cx, |input, _cx| input.set_is_interactive(true));
    }

    fn handle_drag_move(
        &mut self,
        event: &DragMoveEvent<ElementId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if &self.id != event.drag(cx) {
            return;
        }

        let mouse_position = window.mouse_position();
        let delta = self.prev_mouse_pos.map_or(Point::default(), |prev| mouse_position - prev);

        let factor = self.drag_factor();
        self.set_value(self.value(cx) + delta.x.to_f64() * factor, cx);

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
                let slider_width = self.bounds.size.width * px(relative_value as f32);
                div().w(slider_width).h_full().bg(cx.theme().input_slider_bar_color)
            }
            None => div().size_full(),
        };

        InteractiveContainer::new(self.id.clone(), focus_handle)
            .disabled(self.disabled(cx))
            .when(!self.disabled(cx), |e| {
                e.on_click(cx.listener(Self::handle_on_click)).when(is_interactive, |e| {
                    e.cursor_ew_resize()
                        .on_drag(self.id.clone(), |_, _, _, cx| cx.new(|_cx| EmptyView))
                        .on_drag_move(cx.listener(Self::handle_drag_move))
                        .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
                })
            })
            .w_full()
            .flex()
            .child(
                z_stack([
                    slider_bar.into_any_element(),
                    self.input.clone().into_any_element(),
                    bounds_updater(cx.entity(), |this, bounds, _cx| {
                        this.bounds = bounds;
                    })
                    .into_any_element(),
                ])
                .w_full()
                .h(window.line_height()),
            )
    }
}

impl Focusable for NumberField {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.focus_handle(cx)
    }
}

impl EventEmitter<TextInputEvent> for NumberField {}
