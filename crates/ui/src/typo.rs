use gpui::{
    AbsoluteLength, App, Div, ElementId, FontWeight, ParentElement, Stateful, Styled, div,
    prelude::*, px,
};

use crate::{ActiveTheme, InteractiveColor};

pub fn h1(text: &'static str) -> Div {
    div().text_size(AbsoluteLength::Pixels(px(48.0))).font_weight(FontWeight::BOLD).child(text)
}

pub fn h2(text: &'static str) -> Div {
    div().text_size(AbsoluteLength::Pixels(px(36.0))).font_weight(FontWeight::MEDIUM).child(text)
}

pub fn h3(text: &'static str) -> Div {
    div().text_size(AbsoluteLength::Pixels(px(24.0))).font_weight(FontWeight::MEDIUM).child(text)
}

pub fn h4(text: &'static str) -> Div {
    div().text_size(AbsoluteLength::Pixels(px(20.0))).font_weight(FontWeight::MEDIUM).child(text)
}

pub fn h5(text: &'static str) -> Div {
    div().text_size(AbsoluteLength::Pixels(px(16.0))).font_weight(FontWeight::MEDIUM).child(text)
}

pub fn h6(text: &'static str) -> Div {
    div().text_size(AbsoluteLength::Pixels(px(14.0))).font_weight(FontWeight::MEDIUM).child(text)
}

pub fn p(text: &'static str) -> Div {
    div().text_size(AbsoluteLength::Pixels(px(16.0))).font_weight(FontWeight::NORMAL).child(text)
}

pub fn link(
    id: impl Into<ElementId>,
    url: &'static str,
    text: &'static str,
    cx: &App,
) -> Stateful<Div> {
    div()
        .id(id.into())
        .underline()
        .text_color(cx.theme().colors.accent)
        .on_click(|_event, _w, cx| cx.open_url(url))
        .hover(|e| e.text_color(cx.theme().colors.accent.hovered()))
        .cursor_pointer()
        .child(text)
}
