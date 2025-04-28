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

            Show::init(cx, self.showfile_path.as_ref());

            let main_window = MainWindow::open(cx).expect("should open main window");

            ui::init(cx);
            ui::actions::init(cx);
            flow::gpui::actions::init(cx);
            layout::main::actions::init(cx);
            actions::init(main_window, cx);

            self.init_menus(cx);

            let multiverse = cx.new(|_cx| dmx::Multiverse::new());

            self.init_dmx_io(multiverse.clone(), cx);
            output_processor::start(multiverse, cx);
        });
    }

    fn init_dmx_io(&self, multiverse: Entity<dmx::Multiverse>, cx: &mut App) {
        let dmx_io_config = Show::global(cx).dmx_io_settings.clone();
        let dmx_io = DmxIo::new(multiverse.clone(), &dmx_io_config, cx)
            .expect("should create dmx io manager");
        dmx_io.start(cx);
    }

    fn init_menus(&self, cx: &mut App) {
        cx.set_menus(vec![
            Menu {
                name: "".into(),
                items: vec![
                    MenuItem::action("Quit", actions::Quit),
                    MenuItem::action("Settings", layout::main::actions::OpenSettings),
                ],
            },
            Menu {
                name: "File".into(),
                items: vec![
                    MenuItem::action("Save", actions::Save),
                    MenuItem::action("Open", actions::Open),
                ],
            },
        ]);
    }
}

mod actions {
    use anyhow::Context;
    use gpui::*;
    use show::Show;

    use crate::layout::main::MainWindow;

    actions!(app, [Quit, Save, Open, OpenSettings]);

    pub fn init(main_window: WindowHandle<MainWindow>, cx: &mut App) {
        bind_global_keys(cx);
        handle_global_actions(main_window, cx);
    }

    fn bind_global_keys(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("secondary-q", Quit, None)]);
        cx.bind_keys([KeyBinding::new("secondary-s", Save, None)]);
        cx.bind_keys([KeyBinding::new("secondary-o", Open, None)]);
    }

    fn handle_global_actions(main_window: WindowHandle<MainWindow>, cx: &mut App) {
        cx.on_action::<Quit>(move |_, cx| {
            if let Err(err) = handle_quit(main_window, cx) {
                log::error!("Error handling quit: {}", err);
            }
        });

        fn handle_quit(main_window: WindowHandle<MainWindow>, cx: &mut App) -> Result<()> {
            cx.spawn(async move |cx| {
                let answer = main_window.update(cx, |_, w, cx| {
                    w.prompt(
                        PromptLevel::Warning,
                        "Save before exiting?",
                        None,
                        &["Yes", "No", "Cancel"],
                        cx,
                    )
                });

                match answer.unwrap().await {
                    Ok(ix) => match ix {
                        0 => {
                            cx.update(|cx| {
                                cx.dispatch_action(&Save);
                                cx.quit();
                            })
                            .context("update app state")
                            .unwrap();
                        }
                        1 => {
                            cx.update(|cx| {
                                cx.quit();
                            })
                            .context("update app state")
                            .unwrap();
                        }
                        2 => {}
                        _ => {}
                    },
                    Err(err) => {
                        log::error!("Failed to get answer: {}", err);
                    }
                };
            })
            .detach();

            Ok(())
        }

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

        cx.on_action::<Open>(|_, cx| {
            let paths = cx.prompt_for_paths(PathPromptOptions {
                files: true,
                directories: false,
                multiple: false,
            });

            cx.spawn(async move |cx| -> anyhow::Result<()> {
                let Ok(result) = paths.await else {
                    anyhow::bail!("Error awaiting path selection");
                };

                let Ok(Some(paths)) = result else {
                    if let Ok(None) = result {
                        log::info!("Open cancelled - no path selected");
                    } else if let Err(err) = result {
                        anyhow::bail!("Error prompting for path: '{}'", err);
                    }
                    return Ok(());
                };

                let Some(path) = paths.first() else {
                    anyhow::bail!("No path selected");
                };

                log::info!("Closing main window");
                cx.update(|cx| {
                    cx.active_window()
                        .map(|w| {
                            w.update(cx, |_, w, _| {
                                w.remove_window();
                            })
                            .context("removing window")
                        })
                        .context("closing window")
                })???;

                log::info!("Opening show from '{}'", path.display());
                cx.update_global(|show: &mut Show, cx| {
                    Show::open_from_file(path.clone(), cx).map(|new_show| {
                        *show = new_show;
                    })
                })
                .context("Failed to open show")??;

                log::info!("Opening main window");
                cx.update(|cx| {
                    MainWindow::open(cx).expect("should open main window");
                })
                .context("open main window")?;

                log::info!("Show opened successfully");

                Ok(())
            })
            .detach_and_log_err(cx);
        });
    }
}
