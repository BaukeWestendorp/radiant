use gpui::{App, Div, Styled, div, px};

use crate::theme::ActiveTheme;

mod container;
mod root;
mod section;

pub use container::*;
pub use root::*;
pub use section::*;

/// Creates a horizontal divider [Div] using the theme border color.
pub fn h_divider(cx: &App) -> Div {
    div().w_full().h(px(1.0)).bg(cx.theme().border)
}

/// Creates a vertical divider [Div] using the theme border color.
pub fn v_divider(cx: &App) -> Div {
    div().h_full().w(px(1.0)).bg(cx.theme().border)
}
