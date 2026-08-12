use gpui::{App, Div, FontWeight, div};
use gpui::{Window, prelude::*};

use crate::{ActiveTheme, HslaExt, StyledParentExt};

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

pub fn link(text: impl Into<String>, url: &'static str, window: &mut Window, cx: &mut App) -> Div {
    let text = text.into();
    let id = format!("{}-{}", text, url);
    let focus_handle = window
        .use_keyed_state(id.clone(), cx, |_, cx| cx.focus_handle().tab_stop(true))
        .read(cx)
        .clone();

    div().child(
        div()
            .id(id)
            .track_focus(&focus_handle)
            .focus_ring(&focus_handle, window, cx)
            .text_color(cx.theme().accent)
            .hover(|e| e.text_color(cx.theme().accent.hover()))
            .text_decoration_1()
            .cursor_pointer()
            .child(text)
            .on_click(move |_, _, cx| cx.open_url(url)),
    )
}

pub fn sub(text: impl Into<String>, cx: &App) -> Div {
    div().text_xs().italic().text_color(cx.theme().fg_secondary).child(text.into())
}
