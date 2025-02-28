use app::RadiantApp;
use gpui::*;

mod app;
mod frame;
mod graph;

fn main() {
    Application::new().run(|cx: &mut App| {
        ui::theme::Theme::init(cx);

        cx.open_window(WindowOptions { ..Default::default() }, |_, cx| {
            cx.new(|cx| RadiantApp::new(cx))
        })
        .expect("should open window");
    });
}
