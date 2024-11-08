use anyhow::bail;
use gpui::*;
use std::path::PathBuf;
use ui::theme::Theme;

use crate::view;
use crate::view::show::SaveAs;

actions!(app, [Quit, Open]);

pub struct RadiantApp {}

impl RadiantApp {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, cx: &mut AppContext) {
        cx.set_global(Theme::default());

        self.init(cx);
        self.bind_keys(cx);
        self.set_menus(cx);
        self.register_actions(cx);

        cx.activate(true);
    }

    pub fn open_show_window(&self, path: Option<PathBuf>, cx: &mut AppContext) {
        view::show::open_show_window(path, cx).expect("failed to open Show Window");
    }

    fn init(&self, cx: &mut AppContext) {
        ui::init(cx);
        flow_gpui::init(cx);
        view::show::init(cx);
    }

    fn bind_keys(&self, cx: &mut AppContext) {
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-o", Open, None),
        ]);
    }

    fn set_menus(&self, cx: &mut AppContext) {
        cx.set_menus(vec![
            Menu {
                name: "Radiant".to_string().into(),
                items: vec![MenuItem::action("Quit", Quit)],
            },
            Menu {
                name: "File".to_string().into(),
                items: vec![
                    MenuItem::action("Open", Open),
                    MenuItem::action("Save As", SaveAs),
                ],
            },
        ]);
    }

    fn register_actions(&self, cx: &mut AppContext) {
        cx.on_action::<Quit>(|_, cx| {
            cx.quit();
        });

        cx.on_action::<Open>(|_, cx| {
            let paths = cx.prompt_for_paths(PathPromptOptions {
                files: true,
                directories: false,
                multiple: false,
            });

            cx.spawn(|cx| async move {
                let path = match paths.await? {
                    Ok(maybe_paths) => match maybe_paths {
                        Some(paths) => match paths.first().cloned() {
                            Some(path) => path,
                            None => bail!("At least one file needs to be selected"),
                        },
                        None => bail!("Failed to open file: Dialog was cancelled"),
                    },
                    Err(err) => bail!("Failed to open file: {err}"),
                };

                cx.update(|cx| -> Result<()> {
                    view::show::open_show_window(Some(path.clone()), cx)?;
                    cx.add_recent_document(&path);
                    Ok(())
                })??;

                Ok(())
            })
            .detach();
        });
    }
}
