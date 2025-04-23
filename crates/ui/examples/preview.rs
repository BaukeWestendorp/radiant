use gpui::{
    App, Application, Bounds, EmptyView, Entity, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, size,
};
use ui::{Disableable, TabsView, root};

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
        let tabs = vec![
            ui::Tab::new("typography", "Typography", cx.new(|cx| TypographyTab::new(cx)).into()),
            ui::Tab::new("colors", "Colors", cx.new(|_| EmptyView).into()).disabled(true),
            ui::Tab::new("input", "Input", cx.new(|_| EmptyView).into()).disabled(true),
        ];

        Self {
            tab_view: cx.new(|cx| {
                let mut tabs_view = ui::TabsView::new(tabs, w, cx);
                tabs_view.select_tab_ix(0);
                tabs_view
            }),
        }
    }
}

impl Render for ContentView {
    fn render(&mut self, _w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        root(cx).size_full().child(self.tab_view.clone())
    }
}

struct TypographyTab {}

impl TypographyTab {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for TypographyTab {
    fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("typography tab")
    }
}
