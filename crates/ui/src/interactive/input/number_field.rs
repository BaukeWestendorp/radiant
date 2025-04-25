use super::{FieldEvent, TextInput, TextInputEvent};
use crate::{
    ActiveTheme, Disableable, interactive_container,
    utils::{bounds_updater, z_stack},
};
use gpui::*;
use prelude::FluentBuilder;

pub struct NumberField<I: NumberFieldImpl> {
    id: ElementId,
    input: Entity<TextInput>,

    min: Option<f32>,
    max: Option<f32>,
    step: Option<f32>,

    bounds: Bounds<Pixels>,
    prev_mouse_pos: Option<Point<Pixels>>,

    _marker: std::marker::PhantomData<I>,
}

impl<I: NumberFieldImpl + 'static> NumberField<I> {
    pub fn new(
        id: impl Into<ElementId>,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let id = id.into();

        let input = cx.new(|cx| {
            let mut input =
                TextInput::new(id.clone(), focus_handle, window, cx).px(window.rem_size() * 0.25);
            input.interactive(false);
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
                TextInputEvent::Submit(_) => cx.emit(FieldEvent::Submit(this.value(cx))),
                TextInputEvent::Change(_) => cx.emit(FieldEvent::Change(this.value(cx))),
            }
        })
        .detach();

        let mut this = Self {
            id,
            input,

            min: None,
            max: None,
            step: None,

            bounds: Bounds::default(),
            prev_mouse_pos: None,

            _marker: std::marker::PhantomData,
        };

        this.set_min(I::MIN, cx);
        this.set_max(I::MAX, cx);
        this.set_step(I::STEP, cx);
        this.set_value(I::from_f32(0.0), cx);

        this
    }

    pub fn min(&self) -> Option<I::Value> {
        self.min.map(I::from_f32)
    }

    pub fn set_min(&mut self, min: Option<I::Value>, cx: &mut App) {
        if self.min.is_some() && min.is_none() {
            return;
        }

        if self.min.is_some()
            && min.is_some()
            && I::as_f32(min.as_ref().unwrap()) < self.min_as_f32()
        {
            return;
        }

        self.min = min.as_ref().map(I::as_f32);

        self.set_value(self.value(cx), cx);
    }

    fn min_as_f32(&self) -> f32 {
        self.min.unwrap_or(f32::MIN)
    }

    pub fn max(&self) -> Option<I::Value> {
        self.max.map(I::from_f32)
    }

    pub fn set_max(&mut self, max: Option<I::Value>, cx: &mut App) {
        if self.max.is_some() && max.is_none() {
            return;
        }

        if self.max.is_some()
            && max.is_some()
            && I::as_f32(max.as_ref().unwrap()) > self.max_as_f32()
        {
            return;
        }

        self.max = max.as_ref().map(I::as_f32);

        self.set_value(self.value(cx), cx);
    }

    fn max_as_f32(&self) -> f32 {
        self.max.unwrap_or(f32::MAX)
    }

    pub fn step(&self) -> Option<I::Value> {
        self.step.map(I::from_f32)
    }

    pub fn set_step(&mut self, step: Option<I::Value>, cx: &mut App) {
        self.step = step.as_ref().map(I::as_f32);
        self.set_value(self.value(cx), cx);
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

    pub fn value(&self, cx: &App) -> I::Value {
        let value_str = self.input.read(cx).text();
        I::from_str_or_default(value_str)
    }

    pub fn set_value(&mut self, value: I::Value, cx: &mut App) {
        // Clamp
        let min = self.min_as_f32();
        let max = self.max_as_f32();
        let mut value = I::as_f32(&value).clamp(min, max);

        // Step
        if let Some(step) = self.step().map(|v| I::as_f32(&v)) {
            value = (value / step).round() * step;
        }

        // Round
        value = (value * 10e3f32).round() / 10e3f32;

        self.input.update(cx, |text_field, cx| {
            let value_str = value.to_string().into();
            text_field.set_text(value_str, cx);
        })
    }

    fn commit_value(&mut self, cx: &mut App) {
        self.set_value(self.value(cx), cx);
    }

    pub fn is_slider(&self) -> bool {
        self.min().is_some() && self.max().is_some()
    }

    pub fn relative_value(&self, cx: &App) -> Option<f32> {
        if !self.is_slider() {
            return None;
        }

        let min = self.min_as_f32();
        let max = self.max_as_f32();
        let value = I::as_f32(&self.value(cx)).clamp(min, max);
        Some((value - min) / (max - min))
    }

    fn drag_factor(&self) -> f32 {
        if self.is_slider() {
            let delta = self.max_as_f32() - self.min_as_f32();
            delta / self.bounds.size.width.0
        } else {
            0.5
        }
    }
}

impl<I: NumberFieldImpl + 'static> NumberField<I> {
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
        event: &DragMoveEvent<(ElementId, f32, Pixels)>,
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
        let f32_value = start_value + delta_x * factor;
        self.set_value(I::from_f32(f32_value), cx);

        self.prev_mouse_pos = Some(mouse_position);
    }

    fn handle_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _cx: &mut Context<Self>) {
        self.prev_mouse_pos = None;
    }
}

impl<I: NumberFieldImpl + 'static> Render for NumberField<I> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_interactive = !self.input.read(cx).is_interactive();
        let focus_handle = self.input.read(cx).focus_handle(cx);

        let slider_bar = match self.relative_value(cx) {
            Some(relative_value) => {
                div().w(relative(relative_value)).h_full().bg(cx.theme().colors.bg_tertiary)
            }
            None => div().size_full(),
        };

        interactive_container(self.id.clone(), Some(focus_handle))
            .cursor_ew_resize()
            .when(!self.disabled(cx), |e| {
                e.on_click(cx.listener(Self::handle_on_click)).when(is_interactive, |e| {
                    let drag =
                        (self.id.clone(), I::as_f32(&self.value(cx)), window.mouse_position().x);
                    e.on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                        .on_drag(drag, |_, _, _, cx| cx.new(|_cx| EmptyView))
                        .on_drag_move(cx.listener(Self::handle_drag_move))
                        .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
                })
            })
            .w_full()
            .flex()
            .disabled(self.disabled(cx))
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

impl<I: NumberFieldImpl + 'static> EventEmitter<FieldEvent<I::Value>> for NumberField<I> {}

pub trait NumberFieldImpl {
    type Value: Default;

    const MIN: Option<Self::Value>;
    const MAX: Option<Self::Value>;
    const STEP: Option<Self::Value>;

    fn from_str_or_default(s: &str) -> Self::Value;

    fn to_shared_string(value: &Self::Value) -> SharedString;

    fn from_f32(v: f32) -> Self::Value;

    fn as_f32(value: &Self::Value) -> f32;
}

macro_rules! impl_number_field_value {
    ($ty:ty, $min:expr, $max:expr, $step:expr) => {
        impl NumberFieldImpl for $ty {
            type Value = $ty;

            const MIN: Option<Self::Value> = $min;
            const MAX: Option<Self::Value> = $max;
            const STEP: Option<Self::Value> = $step;

            fn from_str_or_default(s: &str) -> Self::Value {
                let f64_value = s.parse::<f64>().unwrap_or_default();
                f64_value.clamp(
                    $min.map(|v: $ty| v as f64).unwrap_or(<$ty>::MIN as f64),
                    $max.map(|v: $ty| v as f64).unwrap_or(<$ty>::MAX as f64),
                ) as Self
            }

            fn to_shared_string(value: &Self::Value) -> SharedString {
                value.to_string().into()
            }

            fn from_f32(v: f32) -> Self::Value {
                v as Self
            }

            fn as_f32(value: &Self::Value) -> f32 {
                *value as f32
            }
        }
    };
}

impl_number_field_value!(f32, None, None, None);
impl_number_field_value!(f64, None, None, None);
impl_number_field_value!(u8, Some(u8::MIN), Some(u8::MAX), Some(1));
impl_number_field_value!(u16, Some(u16::MIN), Some(u16::MAX), Some(1));
impl_number_field_value!(u32, Some(u32::MIN), Some(u32::MAX), Some(1));
impl_number_field_value!(u64, Some(u64::MIN), Some(u64::MAX), Some(1));
impl_number_field_value!(u128, Some(u128::MIN), Some(u128::MAX), Some(1));
impl_number_field_value!(i8, Some(i8::MIN), Some(i8::MAX), Some(1));
impl_number_field_value!(i16, Some(i16::MIN), Some(i16::MAX), Some(1));
impl_number_field_value!(i32, Some(i32::MIN), Some(i32::MAX), Some(1));
impl_number_field_value!(i64, Some(i64::MIN), Some(i64::MAX), Some(1));
impl_number_field_value!(i128, Some(i128::MIN), Some(i128::MAX), Some(1));
