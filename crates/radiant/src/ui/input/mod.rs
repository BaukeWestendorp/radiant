use crate::show::{FloatingDmxValue, patch::FixtureId};
use std::str::FromStr;

mod preset_selector;

pub use preset_selector::*;

impl ui::NumberFieldImpl for FixtureId {
    type Value = FixtureId;

    const MIN: Option<Self::Value> = Some(FixtureId(0));
    const MAX: Option<Self::Value> = None;
    const STEP: Option<f32> = Some(1.0);

    fn from_str_or_default(s: &str) -> Self::Value {
        FixtureId::from_str(s).unwrap_or_default()
    }

    fn to_shared_string(value: &Self::Value) -> gpui::SharedString {
        value.to_string().into()
    }

    fn from_f32(v: f32) -> Self::Value {
        FixtureId(v as u32)
    }

    fn as_f32(value: &Self::Value) -> f32 {
        value.0 as f32
    }
}

impl ui::NumberFieldImpl for FloatingDmxValue {
    type Value = Self;

    const MIN: Option<Self> = Some(FloatingDmxValue(0.0));
    const MAX: Option<Self> = Some(FloatingDmxValue(1.0));
    const STEP: Option<f32> = None;

    fn from_str_or_default(s: &str) -> Self::Value {
        Self(s.parse().unwrap_or_default())
    }

    fn to_shared_string(value: &Self::Value) -> gpui::SharedString {
        value.0.to_string().into()
    }

    fn as_f32(value: &Self::Value) -> f32 {
        value.0
    }

    fn from_f32(v: f32) -> Self::Value {
        Self(v)
    }
}
