use gpui::prelude::*;
use gpui::{App, Div, div};

use crate::theme::ActiveTheme;

pub fn infobar(cx: &App) -> Div {
    div()
        .flex()
        .justify_between()
        .items_center()
        .w_full()
        .min_h_10()
        .max_h_10()
        .px_2()
        .border_t_1()
        .border_color(cx.theme().border)
        .bg(cx.theme().background)
}
