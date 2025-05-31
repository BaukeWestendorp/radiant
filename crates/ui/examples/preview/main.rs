use color::ColorTab;
use gpui::{
    App, Application, Bounds, Entity, Window, WindowBounds, WindowOptions, prelude::*, px, size,
};
use interactive::InteractiveTab;
use misc::MiscTab;
use nav::NavigationTab;
use org::OrganizationTab;
use typo::TypographyTab;
use ui::{TabView, root};

mod color;
mod interactive;
mod misc;
mod nav;
mod org;
mod typo;

fn main() {
    Application::new().run(move |cx: &mut App| {
        cx.activate(true);

        ui::init(cx);
        ui::actions::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(800.0), px(600.0)),
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
    tab_view: Entity<TabView>,
}

impl ContentView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let tabs = vec![
            ui::Tab::new("org", "Organization", cx.new(OrganizationTab::new).into()),
            ui::Tab::new("typo", "Typography", cx.new(TypographyTab::new).into()),
            ui::Tab::new("color", "Colors", cx.new(ColorTab::new).into()),
            ui::Tab::new(
                "interactive",
                "Interactive",
                cx.new(|cx| InteractiveTab::new(window, cx)).into(),
            ),
            ui::Tab::new("nav", "Navigation", cx.new(|cx| NavigationTab::new(window, cx)).into()),
            ui::Tab::new("misc", "Misc", cx.new(MiscTab::new).into()),
        ];

        Self {
            tab_view: cx.new(|cx| {
                let mut tab_view = ui::TabView::new(tabs, window, cx);
                tab_view.select_tab_ix(0);
                tab_view
            }),
        }
    }
}

impl Render for ContentView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        root(cx).size_full().child(self.tab_view.clone())
    }
}
