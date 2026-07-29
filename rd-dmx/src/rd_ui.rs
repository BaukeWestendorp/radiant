use rd_ui::{
    AutoInput, Field, FieldValue, InputState, Slider, SliderValue,
    gpui::{App, AppContext, Entity, Window},
};

use crate::{Address, Channel, UniverseId, Value};

impl SliderValue for Value {
    fn to_f64(&self) -> f64 {
        self.0 as f64
    }

    fn from_f64(value: f64) -> Self {
        let clamped = value.clamp(Self::MIN.to_f64(), Self::MAX.to_f64());
        Self(clamped as u8)
    }

    fn min_value() -> Option<Self> {
        Some(Self::MIN)
    }

    fn max_value() -> Option<Self> {
        Some(Self::MAX)
    }

    fn step_value() -> Option<Self> {
        Some(Self(1))
    }
}

impl AutoInput for Value {
    type Delegate = Slider<Value>;

    fn build_input(
        initial_value: Self,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<InputState<Self::Delegate>> {
        cx.new(move |cx| {
            let slider =
                Slider::new(cx.focus_handle(), window, cx).with_value(Some(initial_value), cx);
            InputState::new(slider, window, cx)
        })
    }
}

impl SliderValue for Channel {
    fn to_f64(&self) -> f64 {
        self.0 as f64
    }

    fn from_f64(value: f64) -> Self {
        let clamped = value.clamp(Self::MIN.to_f64(), Self::MAX.to_f64());
        Self::new_unchecked(clamped as u16)
    }

    fn min_value() -> Option<Self> {
        Some(Self::MIN)
    }

    fn max_value() -> Option<Self> {
        Some(Self::MAX)
    }

    fn step_value() -> Option<Self> {
        Some(Self(1))
    }
}

impl AutoInput for Channel {
    type Delegate = Slider<Channel>;

    fn build_input(
        initial_value: Self,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<InputState<Self::Delegate>> {
        cx.new(move |cx| {
            let slider =
                Slider::new(cx.focus_handle(), window, cx).with_value(Some(initial_value), cx);
            InputState::new(slider, window, cx)
        })
    }
}

impl SliderValue for UniverseId {
    fn to_f64(&self) -> f64 {
        self.0 as f64
    }

    fn from_f64(value: f64) -> Self {
        let clamped = value.clamp(Self::MIN.to_f64(), Self::MAX.to_f64());
        Self::new_unchecked(clamped as u16)
    }

    fn min_value() -> Option<Self> {
        Some(Self::MIN)
    }

    fn max_value() -> Option<Self> {
        Some(Self::MAX)
    }

    fn step_value() -> Option<Self> {
        Some(Self(1))
    }
}

impl AutoInput for UniverseId {
    type Delegate = Slider<UniverseId>;

    fn build_input(
        initial_value: Self,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<InputState<Self::Delegate>> {
        cx.new(move |cx| {
            let slider =
                Slider::new(cx.focus_handle(), window, cx).with_value(Some(initial_value), cx);
            InputState::new(slider, window, cx)
        })
    }
}

impl FieldValue for Address {
    fn from_str(s: &str) -> Option<Self> {
        s.parse().ok()
    }

    fn to_shared_string(&self) -> impl Into<rd_ui::gpui::SharedString> {
        self.to_string()
    }

    fn validator(s: &str) -> bool {
        use std::str::FromStr;

        if s.is_empty() {
            return true;
        }

        let mut parts = s.split('.');
        let first = parts.next().unwrap_or("");
        let second = parts.next();

        if parts.next().is_some() {
            return false;
        }

        if let Some(channel_str) = second {
            let valid_universe = UniverseId::from_str(first).is_ok();
            let valid_channel = channel_str.is_empty() || Channel::from_str(channel_str).is_ok();

            valid_universe && valid_channel
        } else {
            UniverseId::from_str(first).is_ok() || Channel::from_str(first).is_ok()
        }
    }

    fn submit_validator(s: &str) -> bool {
        Self::from_str(s).is_some()
    }
}

impl AutoInput for Address {
    type Delegate = Field<Address>;

    fn build_input(
        initial_value: Self,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<InputState<Self::Delegate>> {
        cx.new(move |cx| {
            let field = Field::new(cx.focus_handle(), window, cx).with_value(initial_value, cx);
            InputState::new(field, window, cx)
        })
    }
}
