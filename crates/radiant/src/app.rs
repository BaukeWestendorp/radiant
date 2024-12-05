use std::path::PathBuf;

use gpui::*;

use crate::workspace::Workspace;

actions!(app, [Save]);

pub struct RadiantApp {
    #[allow(unused)]
    workspace: Model<Workspace>,
}

impl RadiantApp {
    pub fn new(showfile_path: Option<PathBuf>, cx: &mut AppContext) -> anyhow::Result<Self> {
        ui::init(cx);
        flow::gpui::init(cx);

        let workspace = cx.new_model(|cx| Workspace::new(showfile_path.clone(), cx).unwrap());

        cx.activate(true);

        cx.bind_keys([KeyBinding::new("cmd-s", Save, None)]);

        cx.on_action::<Save>({
            let workspace = workspace.clone();
            move |_, cx| {
                let show = workspace.read(cx).show().read(cx).clone();

                if let Some(path) = &showfile_path {
                    show.try_write(path, cx).unwrap();
                } else {
                    log::error!("Show has no path");
                }

                log::info!("Saving show");
            }
        });

        Ok(Self { workspace })
    }
}
