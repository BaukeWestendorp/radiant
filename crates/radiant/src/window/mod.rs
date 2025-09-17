use gpui::prelude::*;
use gpui::{TitlebarOptions, WindowOptions};
use ui::org::{Root, root};

pub mod main;

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

pub fn window_root() -> Root {
    root().flex().flex_col()
}
