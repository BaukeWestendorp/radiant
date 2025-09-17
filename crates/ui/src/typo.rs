use gpui::prelude::*;
use gpui::{
    AbsoluteLength, App, Div, ElementId, FontWeight, ParentElement, SharedString, Stateful, Styled,
    div, px,
};

use crate::theme::{ActiveTheme, InteractiveColor};

/// Heading 1 style (large, bold).
pub fn h1(text: impl Into<SharedString>) -> Div {
    div()
        .text_size(AbsoluteLength::Pixels(px(48.0)))
        .font_weight(FontWeight::BOLD)
        .child(text.into())
}

/// Heading 2 style (slightly smaller, semibold).
pub fn h2(text: impl Into<SharedString>) -> Div {
    div()
        .text_size(AbsoluteLength::Pixels(px(36.0)))
        .font_weight(FontWeight::SEMIBOLD)
        .child(text.into())
}

/// Heading 3 style.
pub fn h3(text: impl Into<SharedString>) -> Div {
    div()
        .text_size(AbsoluteLength::Pixels(px(24.0)))
        .font_weight(FontWeight::SEMIBOLD)
        .child(text.into())
}

/// Heading 4 style.
pub fn h4(text: impl Into<SharedString>) -> Div {
    div()
        .text_size(AbsoluteLength::Pixels(px(20.0)))
        .font_weight(FontWeight::SEMIBOLD)
        .child(text.into())
}

/// Heading 5 style.
pub fn h5(text: impl Into<SharedString>) -> Div {
    div()
        .text_size(AbsoluteLength::Pixels(px(16.0)))
        .font_weight(FontWeight::SEMIBOLD)
        .child(text.into())
}

/// Heading 6 style.
pub fn h6(text: impl Into<SharedString>) -> Div {
    div()
        .text_size(AbsoluteLength::Pixels(px(14.0)))
        .font_weight(FontWeight::SEMIBOLD)
        .child(text.into())
}

/// Paragraph style (normal font).
pub fn p(text: impl Into<SharedString>) -> Div {
    div()
        .text_size(AbsoluteLength::Pixels(px(16.0)))
        .font_weight(FontWeight::NORMAL)
        .child(text.into())
}

/// Styled link that opens a URL when clicked.
pub fn link(
    id: impl Into<ElementId>,
    url: &'static str,
    text: &'static str,
    cx: &App,
) -> Stateful<Div> {
    div()
        .id(id.into())
        .underline()
        .text_color(cx.theme().link)
        .on_click(|_event, _w, cx| cx.open_url(url))
        .hover(|e| e.text_color(cx.theme().link.hovered()))
        .active(|e| e.text_color(cx.theme().link.active()))
        .cursor_pointer()
        .child(text)
}
