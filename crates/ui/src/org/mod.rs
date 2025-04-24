use crate::{ActiveTheme, InteractiveColor};
use gpui::{App, Div, Styled, div, px};

mod container;
mod section;

pub use container::*;
pub use section::*;

pub fn root(cx: &App) -> gpui::Div {
    use gpui::Styled as _;
    gpui::div().text_color(cx.theme().colors.text)
}

pub fn divider(cx: &App) -> Div {
    div().w_full().h(px(1.0)).bg(cx.theme().colors.border.muted())
}
