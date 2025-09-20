use std::any::TypeId;
use std::collections::HashMap;

use gpui::{AnyWindowHandle, App, Global};

mod window;

pub use window::*;

#[derive(Default)]
pub struct WindowManager {
    singleton_windows: HashMap<TypeId, AnyWindowHandle>,
}

impl WindowManager {
    pub fn open_singleton_window<D: WindowDelegate>(&mut self, cx: &mut App) {
        let type_id = TypeId::of::<D>();

        if self.singleton_windows.contains_key(&type_id) {
            self.close_singleton_window::<D>(cx);
        }

        let handle = WindowWrapper::open(cx, |window, cx| D::create(window, cx));
        self.singleton_windows.insert(type_id, handle.into());
    }

    pub fn close_singleton_window<D: WindowDelegate>(&mut self, cx: &mut App) {
        let type_id = TypeId::of::<D>();

        let Some(&handle) = self.singleton_windows.get(&type_id) else {
            // Window already has been closed.
            return;
        };

        // NOTE: We have to defer here, because if we remove the window within the same cycle
        //       as updating the window, the update will fail as the window does not exist anymore.
        cx.defer(move |cx| {
            handle.update(cx, |_, window, _| window.remove_window()).expect("should update window");
        });

        self.singleton_windows.remove(&type_id);
    }
}

impl Global for WindowManager {}
