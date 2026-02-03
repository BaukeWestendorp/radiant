use gpui::{
    AnyView, AnyWindowHandle, App, BorrowAppContext, Global, Window, WindowOptions, prelude::*,
};

use crate::Root;

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
                    cx.on_window_closed(|cx| Self::close(cx)).detach();

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
