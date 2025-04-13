use crate::{
    layout,
    showfile::{Showfile, effect_graph},
};
use gpui::*;
use ui::ActiveTheme;

pub struct RadiantApp {
    main_window: Entity<LayoutWindow>,
}

impl RadiantApp {
    pub fn new(showfile: Showfile, window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.activate(true);

        let effect_graph = cx.new(|_cx| {
            let mut effect_graph = showfile.effect_graph;
            effect_graph::insert_templates(&mut effect_graph);
            effect_graph
        });

        Self { main_window: LayoutWindow::from_showfile(window, cx) }
    }
}

impl Render for RadiantApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(cx.theme().background)
            .text_color(cx.theme().text_primary)
            .child(self.main_window.clone())
    }
}

pub fn run(showfile: Showfile) {
    Application::new().run(|cx: &mut App| {
        ui::init(cx);
        ui::actions::init(cx);
        flow::gpui::actions::init(cx);
        actions::init(cx);

        let _main_window = layout::MainWindow::open(cx);
    });
}

mod actions {
    use gpui::*;

    actions!(app, [Quit]);

    pub fn init(cx: &mut App) {
        bind_global_keys(cx);
        handle_global_actions(cx);
    }

    fn bind_global_keys(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("secondary-q", Quit, None)]);
    }

    fn handle_global_actions(cx: &mut App) {
        cx.on_action::<Quit>(|_, cx| cx.quit());
    }
}
