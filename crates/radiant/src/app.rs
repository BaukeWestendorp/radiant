use crate::{layout::MainWindow, showfile::Showfile};
use gpui::*;

pub struct RadiantApp {
    showfile: Showfile,
}

impl RadiantApp {
    pub fn new(showfile: Showfile) -> Self {
        Self { showfile }
    }

    pub fn run(self) {
        Application::new().run(|cx: &mut App| {
            cx.activate(true);

            ui::init(cx);
            ui::actions::init(cx);
            flow::gpui::actions::init(cx);
            actions::init(cx);

            let _main_window = MainWindow::open(self.showfile.layout.main_window, cx);
        });
    }
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
