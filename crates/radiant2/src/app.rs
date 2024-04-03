use std::path::Path;

use gpui::{point, px, size, AppContext, Bounds, Context, Point, VisualContext, WindowOptions};
use theme::ThemeSettings;

use crate::assets::Assets;
use crate::output::DmxOutputManager;
use crate::showfile::Showfile;
use crate::workspace::Workspace;

pub fn run_app(app: gpui::App, showfile_path: Option<String>) {
    let Some(showfile_path) = showfile_path else {
        todo!("Allow for starting Radiant without a file to load");
    };
    let showfile_path = Path::new(&showfile_path);

    let showfile = match Showfile::from_dir(showfile_path) {
        Ok(showfile) => showfile,
        Err(error) => {
            log::error!(
                "Failed to load showfile at '{}': {error}",
                showfile_path.display()
            );
            // FIXME: Currently we can't show anything if we don't have a showfile.
            todo!("Show a window displaying this error.");
        }
    };

    app.with_assets(Assets).run(move |cx: &mut AppContext| {
        let showfile_model = cx.new_model(|_cx| showfile);

        let window_size = size(1719.into(), 960.into());
        let window_options = WindowOptions {
            bounds: Some(Bounds {
                origin: cx
                    .primary_display()
                    .map(|display| {
                        display.bounds().center()
                            - point(window_size.width / 2, window_size.height / 2)
                    })
                    .unwrap_or(point(1920.into(), 1080.into())),
                size: window_size,
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |cx| {
            ThemeSettings::init(cx);
            DmxOutputManager::init(cx);

            let view = Workspace::build(showfile_model, cx);
            cx.focus_view(&view);
            view
        });
    });
}
