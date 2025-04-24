use super::{TextInput, TextInputEvent};
use crate::{
    ActiveTheme, Disableable, interactive_container,
    utils::{bounds_updater, z_stack},
};
use gpui::*;
use prelude::FluentBuilder;

pub struct NumberField<V>
where
    V: NumberFieldValue,
{
    id: ElementId,
    input: Entity<TextInput>,
    bounds: Bounds<Pixels>,
    prev_mouse_pos: Option<Point<Pixels>>,
    unstepped_value: f32,
    _marker: std::marker::PhantomData<V>,
}

impl<V> NumberField<V>
where
    V: NumberFieldValue + Default + 'static,
{
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

        cx.subscribe(&input, |number_field, input, event, cx| {
            cx.emit(event.clone());
            cx.notify();
            match event {
                TextInputEvent::Blur => {
                    number_field.commit_value(cx);
                    input.update(cx, |input, _cx| input.interactive(false));
                }
                _ => {}
            }
        })
        .detach();

        let mut this = Self {
            id,
            input,
            bounds: Bounds::default(),
            prev_mouse_pos: None,
            unstepped_value: 0.0,
            _marker: Default::default(),
        };

        this.set_value(V::default(), cx);

        this
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

    pub fn value(&self, cx: &App) -> Result<V, V::DeError> {
        let value_str = self.input.read(cx).text();
        V::deserialize(value_str)
    }

    pub fn set_value(&mut self, value: V, cx: &mut App) {
        // Clamp
        let min = V::MIN.map(|v| v.as_f32()).unwrap_or(f32::MIN);
        let max = V::MAX.map(|v| v.as_f32()).unwrap_or(f32::MAX);

        let mut value = value.as_f32().clamp(min, max);
        self.unstepped_value = value;

        // Step
        if let Some(step) = V::STEP.map(|v| v.as_f32()) {
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
        self.set_value(self.value(cx).unwrap_or_default(), cx);
    }

    pub fn is_slider(&self) -> bool {
        V::MIN.is_some() && V::MAX.is_some()
    }

    pub fn relative_value(&self, cx: &App) -> Option<f32> {
        let min = V::MIN?.as_f32();
        let max = V::MAX?.as_f32();
        let value = self.value(cx).unwrap_or_default().as_f32().clamp(min, max);
        Some((value - min) / (max - min))
    }

    fn drag_factor(&self) -> f32 {
        if self.is_slider() {
            let delta = V::MAX.unwrap().as_f32() - V::MIN.unwrap().as_f32();
            delta / self.bounds.size.width.0
        } else {
            0.5
        }
    }
}

impl<V> NumberField<V>
where
    V: NumberFieldValue + Default + 'static,
{
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
        event: &DragMoveEvent<ElementId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if &self.id != event.drag(cx) {
            return;
        }

        let mouse_position = window.mouse_position();
        let delta_x = self.prev_mouse_pos.map_or(px(0.0), |prev| mouse_position.x - prev.x);

        let factor = self.drag_factor();
        self.set_value(V::from_f32(self.unstepped_value + delta_x.0 * factor), cx);

        self.prev_mouse_pos = Some(mouse_position);
    }

    fn handle_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _cx: &mut Context<Self>) {
        self.prev_mouse_pos = None;
    }
}

impl<V> Render for NumberField<V>
where
    V: NumberFieldValue + Default + 'static,
{
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_interactive = !self.input.read(cx).is_interactive();
        let focus_handle = self.input.read(cx).focus_handle(cx);

        let slider_bar = match self.relative_value(cx) {
            Some(relative_value) => {
                let slider_width = self.bounds.size.width * relative_value as f32;
                div().w(slider_width).h_full().bg(cx.theme().colors.bg_tertiary)
            }
            None => div().size_full(),
        };

        interactive_container(self.id.clone(), Some(focus_handle))
            .cursor_ew_resize()
            .when(!self.disabled(cx), |e| {
                e.on_click(cx.listener(Self::handle_on_click)).when(is_interactive, |e| {
                    e.on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                        .on_drag(self.id.clone(), |_, _, _, cx| cx.new(|_cx| EmptyView))
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

impl<V> Focusable for NumberField<V>
where
    V: NumberFieldValue + 'static,
{
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.focus_handle(cx)
    }
}

impl<V> EventEmitter<TextInputEvent> for NumberField<V> where V: NumberFieldValue + 'static {}

pub trait NumberFieldValue: Sized {
    type DeError;

    const MIN: Option<Self>;
    const MAX: Option<Self>;
    const STEP: Option<Self>;

    fn serialize(value: Self) -> SharedString;

    fn deserialize(string: &SharedString) -> Result<Self, Self::DeError>
    where
        Self: Sized;

    fn from_f32(f: f32) -> Self;

    fn as_f32(&self) -> f32;
}

macro_rules! impl_number_field_value {
    ($t:ty, $err:ty, $step:expr) => {
        impl NumberFieldValue for $t {
            type DeError = $err;

            const MIN: Option<Self> = Some(<$t>::MIN);
            const MAX: Option<Self> = Some(<$t>::MAX);
            const STEP: Option<Self> = Some($step);

            fn serialize(value: Self) -> SharedString {
                value.to_string().into()
            }

            fn deserialize(string: &SharedString) -> Result<Self, Self::DeError> {
                string.parse()
            }

            fn from_f32(f: f32) -> Self {
                f as $t
            }

            fn as_f32(&self) -> f32 {
                *self as f32
            }
        }
    };
}

impl_number_field_value!(i8, std::num::ParseIntError, 1);
impl_number_field_value!(i16, std::num::ParseIntError, 1);
impl_number_field_value!(i32, std::num::ParseIntError, 1);
impl_number_field_value!(i64, std::num::ParseIntError, 1);
impl_number_field_value!(i128, std::num::ParseIntError, 1);
impl_number_field_value!(isize, std::num::ParseIntError, 1);
impl_number_field_value!(u8, std::num::ParseIntError, 1);
impl_number_field_value!(u16, std::num::ParseIntError, 1);
impl_number_field_value!(u32, std::num::ParseIntError, 1);
impl_number_field_value!(u64, std::num::ParseIntError, 1);
impl_number_field_value!(u128, std::num::ParseIntError, 1);
impl_number_field_value!(usize, std::num::ParseIntError, 1);
impl_number_field_value!(f32, std::num::ParseFloatError, 1.0);
impl_number_field_value!(f64, std::num::ParseFloatError, 1.0);
