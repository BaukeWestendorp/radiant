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
    fn render(&mut self, _w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let headers = div()
            .child(ui::h1("Header Level 1"))
            .child(ui::h2("Header Level 2"))
            .child(ui::h3("Header Level 3"))
            .child(ui::h4("Header Level 4"))
            .child(ui::h5("Header Level 5"))
            .child(ui::h6("Header Level 6"));

        let paragraphs = div()
            .child(ui::p("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."));

        let links =
            div().child(ui::link("example-link", "https://example.com", "Example Link", cx));

        div()
            .p_2()
            .child(ui::section("Headers", headers.mb_4(), cx))
            .child(ui::section("Paragraphs", paragraphs.mb_4(), cx))
            .child(ui::section("Links", links.mb_4(), cx))
    }
}
