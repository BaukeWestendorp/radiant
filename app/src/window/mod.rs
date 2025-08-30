use gpui::prelude::*;
use gpui::{App, TitlebarOptions, Window, WindowOptions};
use ui::misc::titlebar;
use ui::org::{Root, root};

pub mod patch;

pub fn window_options() -> WindowOptions {
    WindowOptions {
        window_bounds: None,
        titlebar: Some(TitlebarOptions {
            title: Some("Radiant".into()),
            appears_transparent: true,
            traffic_light_position: Some(ui::misc::TRAFFIC_LIGHT_POSITION),
        }),
        app_id: Some("radiant".to_string()),
        ..Default::default()
    }
}

pub fn window_root(window: &mut Window, cx: &mut App) -> Root {
    root().flex().flex_col().child(titlebar(window, cx))
}
