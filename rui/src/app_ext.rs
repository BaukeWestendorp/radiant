use gpui::{AnyView, App, Window, WindowOptions};

use crate::settings::SettingsAgent;

pub trait AppExt {
    fn open_settings<F: Fn(&mut Window, &mut App) -> AnyView>(
        &mut self,
        window_options: Option<WindowOptions>,
        root_view_builder: F,
    );

    fn close_settings(&mut self);
}

impl AppExt for App {
    fn open_settings<F: Fn(&mut Window, &mut App) -> AnyView>(
        &mut self,
        window_options: Option<WindowOptions>,
        root_view_builder: F,
    ) {
        SettingsAgent::open(window_options, self, root_view_builder);
    }

    fn close_settings(&mut self) {
        SettingsAgent::close(self);
    }
}
