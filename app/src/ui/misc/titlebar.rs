use gpui::prelude::*;
use gpui::{App, Div, Pixels, Point, Stateful, Window, div, px};

use crate::ui::ActiveTheme;

pub const TRAFFIC_LIGHT_PADDING: Pixels = px(71.0);
pub const TRAFFIC_LIGHT_POSITION: Point<Pixels> = Point::new(px(9.0), px(9.0));

pub fn titlebar(window: &Window, cx: &App) -> Stateful<Div> {
    let titlebar_height = (1.75 * window.rem_size()).max(px(34.));

    div()
        .id("titlebar")
        .window_control_area(gpui::WindowControlArea::Drag)
        .w_full()
        .min_h(titlebar_height)
        .max_h(titlebar_height)
        .pl(TRAFFIC_LIGHT_PADDING)
        .border_b_1()
        .border_color(cx.theme().colors.border)
        .bg(cx.theme().colors.bg_secondary)
        .flex()
        .items_center()
        .on_click(|event, window, _| {
            if event.down.click_count == 2 {
                window.titlebar_double_click();
            }
        })
}
