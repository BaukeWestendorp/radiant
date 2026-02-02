use gpui::{Div, Styled, div};

/// Returns a `Div` as horizontal flex layout.
#[inline(always)]
pub fn h_flex() -> Div {
    div().h_flex()
}

/// Returns a `Div` as vertical flex layout.
#[inline(always)]
pub fn v_flex() -> Div {
    div().v_flex()
}

pub trait StyledExt: Styled + Sized {
    /// Apply self into a horizontal flex layout.
    #[inline(always)]
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    /// Apply self into a vertical flex layout.
    #[inline(always)]
    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }
}

impl<E: Styled> StyledExt for E {}
