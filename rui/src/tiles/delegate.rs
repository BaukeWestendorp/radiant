use gpui::{AnyElement, App, Window};

pub trait TileDelegate {
    fn title(&self) -> &str;

    fn render_content(&self, window: &mut Window, cx: &App) -> AnyElement;

    fn show_header(&self, _cx: &App) -> bool {
        true
    }
}
