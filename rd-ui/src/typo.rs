use gpui::prelude::*;
use gpui::{App, Div, FontWeight, MouseButton, div};

use crate::{ActiveTheme, HslaExt};

pub fn article() -> Div {
    div().size_full().flex().flex_col().gap_2()
}

pub fn h1(text: impl Into<String>, cx: &App) -> Div {
    div()
        .text_3xl()
        .font_weight(FontWeight::BOLD)
        .text_color(cx.theme().fg_primary)
        .child(text.into())
}

pub fn h2(text: impl Into<String>, cx: &App) -> Div {
    div()
        .text_2xl()
        .font_weight(FontWeight::BOLD)
        .text_color(cx.theme().fg_primary)
        .child(text.into())
}

pub fn h3(text: impl Into<String>, cx: &App) -> Div {
    div()
        .text_xl()
        .font_weight(FontWeight::BOLD)
        .text_color(cx.theme().fg_primary)
        .child(text.into())
}

pub fn h4(text: impl Into<String>, cx: &App) -> Div {
    div()
        .text_lg()
        .font_weight(FontWeight::BOLD)
        .text_color(cx.theme().fg_primary)
        .child(text.into())
}

pub fn h5(text: impl Into<String>, cx: &App) -> Div {
    div()
        .text_base()
        .font_weight(FontWeight::BOLD)
        .text_color(cx.theme().fg_primary)
        .child(text.into())
}

pub fn h6(text: impl Into<String>, cx: &App) -> Div {
    div()
        .text_base()
        .font_weight(FontWeight::MEDIUM)
        .text_color(cx.theme().fg_primary)
        .child(text.into())
}

pub fn link(text: impl Into<String>, url: &'static str, cx: &App) -> Div {
    let text = text.into();
    div().child(
        div()
            .id(format!("{}-{}", text, url))
            .text_color(cx.theme().accent)
            .hover(|e| e.text_color(cx.theme().accent.hover()))
            .text_decoration_1()
            .cursor_pointer()
            .child(text)
            .on_mouse_down(MouseButton::Left, move |_, _, cx| cx.open_url(url)),
    )
}

pub fn sub(text: impl Into<String>, cx: &App) -> Div {
    div().text_xs().italic().text_color(cx.theme().fg_secondary).child(text.into())
}
