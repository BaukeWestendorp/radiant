use std::path::PathBuf;

use crate::layout::MainWindow;
use gpui::*;
use show::Show;

pub const APP_ID: &str = "radiant";

pub struct RadiantApp {
    showfile_path: Option<PathBuf>,
}

impl RadiantApp {
    pub fn new(showfile_path: Option<PathBuf>) -> Self {
        Self { showfile_path }
    }

    pub fn run(self) {
        Application::new().run(|cx: &mut App| {
            cx.activate(true);

            ui::init(cx);
            ui::actions::init(cx);
            flow::gpui::actions::init(cx);
            actions::init(cx);

            let show = match self.showfile_path {
                Some(path) => match show::open_from_file(&path, cx) {
                    Ok(show) => show,
                    Err(err) => {
                        log::error!("Error opening showfile: {}", err);
                        std::process::exit(1);
                    }
                },
                None => Show::default(),
            };

            cx.set_global(show);

            MainWindow::open(cx).expect("should open main window");
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
