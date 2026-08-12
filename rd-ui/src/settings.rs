use gpui::{
    AnyView, AnyWindowHandle, App, BorrowAppContext, FontWeight, Global, TitlebarOptions, Window,
    WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions, div, px, size,
};
use gpui::{SharedString, prelude::*};

use crate::{ActiveTheme, Root, comp::TitleBar, h_flex, v_flex};

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

                    cx.new(|cx| {
                        Root::new(
                            cx.new(|cx| SettingsRoot::new(window, cx, build_root_view)),
                            window,
                            cx,
                        )
                    })
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

pub struct SettingsRoot {
    view: AnyView,
}

impl SettingsRoot {
    pub fn new<F: Fn(&mut Window, &mut App) -> AnyView>(
        window: &mut Window,
        cx: &mut Context<Self>,
        build_content: F,
    ) -> Self {
        Self { view: ((build_content)(window, cx)) }
    }

    fn render_title_bar_content(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex().size_full().justify_between().child(
            div()
                .font_weight(FontWeight::BOLD)
                .text_color(cx.theme().fg_secondary)
                .child(window.window_title()),
        )
    }

    fn render_content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        self.view.clone()
    }
}

impl Render for SettingsRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(TitleBar::new().child(self.render_title_bar_content(window, cx)))
            .child(div().size_full().overflow_hidden().child(self.render_content(window, cx)))
    }
}

pub fn settings_window_options(cx: &App) -> WindowOptions {
    WindowOptions {
        titlebar: Some(TitlebarOptions {
            title: Some(SharedString::new_static("Settings")),
            appears_transparent: true,
            traffic_light_position: None,
        }),
        window_bounds: Some(WindowBounds::centered(size(px(1080.0), px(720.0)), cx)),
        focus: true,
        show: true,
        kind: WindowKind::Normal,
        is_movable: true,
        is_resizable: true,
        is_minimizable: false,
        display_id: None,
        window_background: WindowBackgroundAppearance::Opaque,
        app_id: None,
        window_min_size: None,
        window_decorations: None,
        icon: None,
        tabbing_identifier: None,
    }
}
