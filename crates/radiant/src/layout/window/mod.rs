use gpui::{App, prelude::*};

pub mod main;
pub mod settings;

pub const DEFAULT_REM_SIZE: gpui::Pixels = gpui::px(14.0);

pub trait VirtualWindow: Render {
    fn title(&self, cx: &App) -> &str;

    fn show_title_bar(&self, _cx: &App) -> bool {
        true
    }
}
