use crate::{
    dmx_io::DmxIo,
    layout::{self, main::MainWindow},
    output_processor,
};
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

            self.init_show(cx);

            ui::init(cx);
            ui::actions::init(cx);
            flow::gpui::actions::init(cx);
            layout::main::actions::init(cx);
            actions::init(cx);

            let multiverse = cx.new(|_cx| dmx::Multiverse::new());

            self.init_dmx_io(multiverse.clone(), cx);
            output_processor::start(multiverse, cx);

            MainWindow::open(cx).expect("should open main window");
        });
    }

    fn init_show(&self, cx: &mut App) {
        let show = match &self.showfile_path {
            Some(path) => match Show::open_from_file(path.clone(), cx) {
                Ok(show) => show,
                Err(err) => {
                    log::error!("Error opening showfile: '{}'", err);
                    std::process::exit(1);
                }
            },
            None => Show::default(),
        };

        cx.set_global(show);
    }

    fn init_dmx_io(&self, multiverse: Entity<dmx::Multiverse>, cx: &mut App) {
        let dmx_io_config = Show::global(cx).dmx_io_settings.clone();
        let dmx_io = DmxIo::new(multiverse.clone(), &dmx_io_config, cx)
            .expect("should create dmx io manager");
        dmx_io.start(cx);
    }
}

mod actions {
    use gpui::*;
    use show::Show;

    actions!(app, [Quit, Save, OpenSettings]);

    pub fn init(cx: &mut App) {
        bind_global_keys(cx);
        handle_global_actions(cx);
    }

    fn bind_global_keys(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("secondary-q", Quit, None)]);
        cx.bind_keys([KeyBinding::new("secondary-s", Save, None)]);
        cx.bind_keys([KeyBinding::new("secondary-,", OpenSettings, None)]);
    }

    fn handle_global_actions(cx: &mut App) {
        cx.on_action::<Quit>(|_, cx| cx.quit());

        cx.on_action::<Save>(|_, cx| {
            let path = Show::global(cx).path.clone();
            if let Some(path) = &path {
                log::info!("Saving show to '{}'", path.display());
                match Show::update_global(cx, |show, cx| show.save_to_file(path, cx)) {
                    Ok(_) => log::info!("Show saved successfully"),
                    Err(err) => log::error!("Error saving show: '{}'", err),
                }
                return;
            }

            let Some(dir) = dirs::home_dir() else {
                log::error!("Could not determine home directory");
                return;
            };

            let path = cx.prompt_for_new_path(&dir.to_path_buf());
            cx.spawn(async move |cx| {
                let Ok(result) = path.await else {
                    log::error!("Error awaiting path selection");
                    return;
                };

                let Ok(Some(path)) = result else {
                    if let Ok(None) = result {
                        log::info!("Save cancelled - no path selected");
                    } else if let Err(err) = result {
                        log::error!("Error prompting for path: '{}'", err);
                    }
                    return;
                };

                log::info!("Saving show to '{}'", path.display());
                let result = cx.update_global(|show: &mut Show, cx| show.save_to_file(&path, cx));

                match result {
                    Ok(Ok(_)) => log::info!("Show saved successfully"),
                    Ok(Err(err)) => log::error!("Error saving show: '{}'", err),
                    Err(err) => log::error!("Error updating global show: '{}'", err),
                }
            })
            .detach();
        });
    }
}
