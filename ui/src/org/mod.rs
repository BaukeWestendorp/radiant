use gpui::{App, Div, Styled, div, px};

use crate::ActiveTheme;

mod container;
mod section;

pub use container::*;
pub use section::*;

pub fn root(cx: &App) -> gpui::Div {
    use gpui::Styled as _;
    gpui::div().text_color(cx.theme().colors.text)
}

pub fn h_divider(cx: &App) -> Div {
    div().w_full().h(px(1.0)).bg(cx.theme().colors.border)
}

pub fn v_divider(cx: &App) -> Div {
    div().h_full().w(px(1.0)).bg(cx.theme().colors.border)
}
