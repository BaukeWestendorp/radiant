use std::{fmt, num::NonZeroU32};

use uuid::Uuid;

pub use executor::*;

mod executor;

pub trait Object: for<'facet> facet::Facet<'facet> {
    fn slot(&self) -> Slot;

    fn id(&self) -> ObjectId;

    fn name(&self) -> &str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(facet::Facet)]
#[facet(transparent)]
pub struct ObjectId(Uuid);

impl ObjectId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn random() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "rd-ui")]
impl rd_ui::FieldValue for ObjectId {
    fn from_str(s: &str) -> Option<Self> {
        Uuid::parse_str(s).ok().map(Self)
    }

    fn to_shared_string(&self) -> impl Into<rd_ui::gpui::SharedString> {
        self.0.to_string()
    }

    fn validator(s: &str) -> bool {
        Uuid::parse_str(s).is_ok()
    }

    fn submit_validator(s: &str) -> bool {
        Uuid::parse_str(s).is_ok()
    }
}

#[cfg(feature = "rd-ui")]
impl rd_ui::AutoInput for ObjectId {
    type Delegate = rd_ui::Field<Self>;

    fn build_input(
        initial_value: Self,
        window: &mut rd_ui::gpui::Window,
        cx: &mut rd_ui::gpui::App,
    ) -> rd_ui::gpui::Entity<rd_ui::InputState<Self::Delegate>> {
        use rd_ui::gpui::AppContext as _;
        cx.new(|cx| {
            let field =
                rd_ui::Field::new(cx.focus_handle(), window, cx).with_value(initial_value, cx);
            rd_ui::InputState::new(field, window, cx)
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(facet::Facet)]
#[facet(transparent)]
pub struct Slot(NonZeroU32);

impl Slot {
    pub fn new(nz: NonZeroU32) -> Self {
        Self(nz)
    }

    pub fn as_u32(&self) -> u32 {
        self.0.into()
    }
}

impl Default for Slot {
    fn default() -> Self {
        Self(NonZeroU32::new(1).unwrap())
    }
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "rd-ui")]
impl rd_ui::FieldValue for Slot {
    fn from_str(s: &str) -> Option<Self> {
        s.parse::<u32>().ok().and_then(|n| NonZeroU32::new(n)).map(Self)
    }

    fn to_shared_string(&self) -> impl Into<rd_ui::gpui::SharedString> {
        self.0.get().to_string()
    }

    fn validator(s: &str) -> bool {
        s.parse::<u32>().ok().and_then(|n| NonZeroU32::new(n)).is_some()
    }

    fn submit_validator(s: &str) -> bool {
        s.parse::<u32>().ok().and_then(|n| NonZeroU32::new(n)).is_some()
    }
}

#[cfg(feature = "rd-ui")]
impl rd_ui::AutoInput for Slot {
    type Delegate = rd_ui::Field<Self>;

    fn build_input(
        initial_value: Self,
        window: &mut rd_ui::gpui::Window,
        cx: &mut rd_ui::gpui::App,
    ) -> rd_ui::gpui::Entity<rd_ui::InputState<Self::Delegate>> {
        use rd_ui::gpui::AppContext as _;
        cx.new(|cx| {
            let field =
                rd_ui::Field::new(cx.focus_handle(), window, cx).with_value(initial_value, cx);
            rd_ui::InputState::new(field, window, cx)
        })
    }
}
