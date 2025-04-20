use crate::{dmx_io::DmxIo, layout::MainWindow, output_processor};
use gpui::*;
use show::Show;
use std::path::PathBuf;

pub const APP_ID: &str = "radiant";

pub struct RadiantApp {
    showfile_path: Option<PathBuf>,
}

impl RadiantApp {
    pub fn new(showfile_path: Option<PathBuf>) -> Self {
        Self { showfile_path }
    }

    pub fn run(self) {
        Application::new().run(move |cx: &mut App| {
            cx.activate(true);

            ui::init(cx);
            ui::actions::init(cx);
            flow::gpui::actions::init(cx);
            actions::init(cx);

            let multiverse = cx.new(|_cx| dmx::Multiverse::new());

            self.init_show(cx);
            self.init_dmx_io(multiverse.clone(), cx);
            output_processor::start(multiverse, cx);

            MainWindow::open(cx).expect("should open main window");
        });
    }

    fn init_show(&self, cx: &mut App) {
        let show = match &self.showfile_path {
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
    }

    fn init_dmx_io(&self, multiverse: Entity<dmx::Multiverse>, cx: &mut App) {
        let dmx_io_config = Show::global(cx).dmx_io_settings.clone();
        let dmx_io =
            DmxIo::new(multiverse.clone(), &dmx_io_config).expect("should create dmx io manager");
        dmx_io.start(cx);
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
