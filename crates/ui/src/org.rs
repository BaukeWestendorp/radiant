use crate::{ActiveTheme, InteractiveColor};
use gpui::{App, Div, div, prelude::*, px};

pub fn root(cx: &App) -> gpui::Div {
    use gpui::Styled as _;
    gpui::div().text_color(cx.theme().text_primary)
}

pub fn divider(cx: &App) -> Div {
    div().w_full().h(px(1.0)).bg(cx.theme().border.muted())
}

pub fn section(title: &'static str, content: impl IntoElement, cx: &App) -> Div {
    let header = div()
        .w_full()
        .flex()
        .items_center()
        .gap_2()
        .text_color(cx.theme().text_primary.muted())
        .child(title)
        .child(divider(cx));

    div().child(header).child(content)
}

pub fn container(content: impl IntoElement, cx: &App) -> Div {
    div()
        .bg(cx.theme().element_background)
        .border_1()
        .border_color(cx.theme().border.muted())
        .rounded(cx.theme().radius)
        .child(content)
}
