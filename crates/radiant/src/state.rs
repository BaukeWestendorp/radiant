use std::any::TypeId;
use std::collections::HashMap;

use gpui::{AnyWindowHandle, App, AppContext, Global, UpdateGlobal, WindowHandle};

use crate::window::settings::SettingsWindow;

#[derive(Default)]
pub struct AppState {
    opened_secondary_windows: HashMap<TypeId, AnyWindowHandle>,
}

impl AppState {
    pub fn init(cx: &mut App) {
        cx.set_global(Self::default())
    }

    pub fn open_settings(cx: &mut App) {
        Self::open_window(SettingsWindow::open(cx), cx);
    }

    pub fn close_settings(cx: &mut App) {
        Self::close_window::<SettingsWindow>(cx);
    }

    pub fn close_all_windows(cx: &mut App) {
        Self::update_global(cx, |this, cx| {
            for (_, handle) in this.opened_secondary_windows.drain() {
                Self::close_any_window(handle, cx);
            }
        });
    }

    fn open_window<T: 'static>(handle: WindowHandle<T>, cx: &mut App) {
        Self::close_settings(cx);

        cx.defer(move |cx| {
            Self::update_global(cx, |this, _| {
                this.opened_secondary_windows.insert(TypeId::of::<T>(), handle.into());
            });
        });
    }

    fn close_window<T: 'static>(cx: &mut App) {
        cx.defer(move |cx| {
            Self::update_global(cx, |this, cx| {
                if let Some(handle) = this.opened_secondary_windows.remove(&TypeId::of::<T>()) {
                    Self::close_any_window(handle.into(), cx);
                }
            });
        });
    }

    fn close_any_window(handle: AnyWindowHandle, cx: &mut App) {
        cx.update_window(handle, |_, window, _| window.remove_window()).ok();
    }
}

impl Global for AppState {}
