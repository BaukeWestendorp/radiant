use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    size,
};

use rui::Root;

struct RadiantApp {}

impl RadiantApp {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for RadiantApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child("Radiant App")
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        rui::init(cx);

        cx.activate(true);

        let bounds = Bounds::centered(None, size(px(1080.0), px(720.0)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };

        cx.open_window(options, |window, cx| {
            let view = cx.new(|cx| RadiantApp::new(cx));
            cx.new(|cx| Root::new(view, window, cx))
        })
        .expect("should open main window");
    });
}
