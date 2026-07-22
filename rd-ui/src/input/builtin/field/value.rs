use gpui::{App, SharedString, Window, prelude::*};

pub trait FieldValue: Clone {
    fn from_str(s: &str) -> Option<Self>
    where
        Self: Sized;

    fn to_shared_string(&self) -> impl Into<SharedString>;

    fn validator(s: &str) -> bool;

    fn submit_validator(s: &str) -> bool;

    fn render_overlay(_window: &mut Window, _cx: &mut App) -> Option<impl IntoElement> {
        Option::<gpui::Empty>::None
    }
}

macro_rules! impl_field_value_parse {
    ($($t:ty),*) => {
        $(
            impl FieldValue for $t {
                fn from_str(s: &str) -> Option<Self> {
                    s.parse().ok()
                }

                fn to_shared_string(&self) -> impl Into<SharedString> {
                    self.to_string()
                }

                fn validator(s: &str) -> bool {
                    s.parse::<$t>().is_ok()
                }

                fn submit_validator(s: &str) -> bool {
                    s.parse::<$t>().is_ok()
                }
            }
        )*
    };
}

impl_field_value_parse!(
    f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, bool
);

macro_rules! impl_field_value_string {
    ($($t:ty),*) => {
        $(
            impl FieldValue for $t {
                fn from_str(s: &str) -> Option<Self> {
                    Some(s.into())
                }

                fn to_shared_string(&self) -> impl Into<SharedString> {
                    self.clone()
                }

                fn validator(s: &str) -> bool {
                    !s.trim().is_empty()
                }

                fn submit_validator(s: &str) -> bool {
                    !s.trim().is_empty()
                }
            }
        )*
    };
}

impl_field_value_string!(SharedString, String);

impl<T: FieldValue> FieldValue for Option<T> {
    fn from_str(s: &str) -> Option<Self> {
        if s.trim().is_empty() { Some(None) } else { T::from_str(s).map(Some) }
    }

    fn to_shared_string(&self) -> impl Into<SharedString> {
        match self {
            Some(value) => value.to_shared_string().into(),
            None => SharedString::default(),
        }
    }

    fn validator(s: &str) -> bool {
        if s.trim().is_empty() { true } else { T::validator(s) }
    }

    fn submit_validator(s: &str) -> bool {
        if s.trim().is_empty() { true } else { T::submit_validator(s) }
    }
}
