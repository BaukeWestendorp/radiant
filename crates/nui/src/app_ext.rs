use gpui::{App, BorrowAppContext};

use crate::WindowManager;

pub trait AppExt {
    fn wm(&self) -> &WindowManager;
    fn update_wm<R, F: FnOnce(&mut WindowManager, &mut App) -> R>(&mut self, f: F) -> R;
}

impl AppExt for App {
    fn wm(&self) -> &WindowManager {
        self.global()
    }

    fn update_wm<R, F: FnOnce(&mut WindowManager, &mut App) -> R>(&mut self, f: F) -> R {
        self.update_global(f)
    }
}
