use gpui::prelude::*;
use gpui::{SharedString, TitlebarOptions, WindowOptions};
use ui::org::{Root, root};

pub mod main;
pub mod settings;

pub fn window_options(title: impl Into<SharedString>) -> WindowOptions {
    WindowOptions {
        window_bounds: None,
        titlebar: Some(TitlebarOptions {
            title: Some(title.into()),
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
