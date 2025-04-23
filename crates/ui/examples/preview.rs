use gpui::{
    App, Application, Bounds, Entity, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    size,
};
use ui::TabsView;

fn main() {
    Application::new().run(move |cx: &mut App| {
        cx.activate(true);

        ui::init(cx);
        ui::actions::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(1600.0), px(960.0)),
                cx,
            ))),
            app_id: Some("radiant".to_string()),
            ..Default::default()
        };

        cx.open_window(window_options, |w, cx| cx.new(|cx| ContentView::new(w, cx)))
            .expect("open main window");
    });
}

struct ContentView {
    tab_view: Entity<TabsView>,
}

impl ContentView {
    pub fn new(w: &mut Window, cx: &mut Context<Self>) -> Self {
        let tabs =
            vec![ui::Tab { label: todo!(), id: todo!(), disabled: todo!(), content: todo!() }];

        Self { tab_view: cx.new(|cx| ui::TabsView::new(tabs, w, cx)) }
    }
}

impl Render for ContentView {
    fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.tab_view.clone())
    }
}
