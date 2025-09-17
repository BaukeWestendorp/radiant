use gpui::prelude::*;
use gpui::{App, Div, FontWeight, Pixels, Point, Stateful, Window, WindowControlArea, div, px};

use crate::theme::ActiveTheme;

pub const TRAFFIC_LIGHT_PADDING: Pixels = px(69.0);
pub const TRAFFIC_LIGHT_POSITION: Point<Pixels> = Point::new(px(8.0), px(8.0));

pub fn titlebar(window: &Window, cx: &App) -> Stateful<Div> {
    let titlebar_height = px(33.0);

    div()
        .id("titlebar")
        .window_control_area(WindowControlArea::Drag)
        .w_full()
        .min_h(titlebar_height)
        .max_h(titlebar_height)
        .pl(TRAFFIC_LIGHT_PADDING)
        .pr(TRAFFIC_LIGHT_POSITION.x)
        .border_b_1()
        .border_color(cx.theme().title_bar_border)
        .bg(cx.theme().title_bar)
        .flex()
        .items_center()
        .child(div().font_weight(FontWeight::BOLD).pb(px(-2.0)).child(window.window_title()))
        .on_click(|event, window, _| {
            if event.click_count() == 2 {
                window.titlebar_double_click();
            }
        })
}
