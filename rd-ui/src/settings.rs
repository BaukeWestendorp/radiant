use gpui::{AnyView, AnyWindowHandle, App, BorrowAppContext, Global, Window, WindowOptions};
use gpui::{SharedString, prelude::*};

use crate::Root;

pub const SETTINGS_WINDOW_OPTIONS: WindowOptions = WindowOptions {
    titlebar: Some(gpui::TitlebarOptions {
        title: Some(SharedString::new_static("Settings")),
        appears_transparent: true,
        traffic_light_position: None,
    }),
    window_bounds: None,
    focus: true,
    show: true,
    kind: gpui::WindowKind::Floating,
    is_movable: true,
    is_resizable: true,
    is_minimizable: false,
    display_id: None,
    window_background: gpui::WindowBackgroundAppearance::Opaque,
    app_id: None,
    window_min_size: None,
    window_decorations: None,
    icon: None,
    tabbing_identifier: None,
};

pub(crate) fn init(cx: &mut App) {
    cx.set_global(SettingsAgent::default())
}

#[derive(Default)]
pub(crate) struct SettingsAgent {
    window_handle: Option<AnyWindowHandle>,
}

impl SettingsAgent {
    pub fn open<F: Fn(&mut Window, &mut App) -> AnyView>(
        window_options: Option<WindowOptions>,
        cx: &mut App,
        build_root_view: F,
    ) {
        cx.update_global(|this: &mut Self, cx| {
            if this.window_handle.is_some() {
                log::debug!("settings window already opened");
                return;
            }

            let handle = cx
                .open_window(window_options.unwrap_or_default(), |window, cx| {
                    cx.on_window_closed(|cx, _| Self::close(cx)).detach();

                    cx.new(|cx| Root::new((build_root_view)(window, cx), window, cx))
                })
                .expect("should open settings window");

            this.window_handle = Some(handle.into());
        });
    }

    pub fn close(cx: &mut App) {
        cx.update_global(|this: &mut Self, cx| {
            let Some(window_handle) = this.window_handle.take() else { return };

            let _ = window_handle.update(cx, |_, window, _cx| {
                window.remove_window();
            });
        });
    }
}

impl Global for SettingsAgent {}

pub trait SettingsAppExt {
    fn open_settings<F: Fn(&mut Window, &mut App) -> AnyView>(
        &mut self,
        window_options: Option<WindowOptions>,
        root_view_builder: F,
    );

    fn close_settings(&mut self);
}

impl SettingsAppExt for App {
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
