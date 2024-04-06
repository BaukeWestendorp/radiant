use gpui::{point, size, AppContext, Bounds, VisualContext, WindowOptions};
use theme::ThemeSettings;

use crate::assets::Assets;
use crate::output::DmxOutputManager;
use crate::showfile::ShowfileManager;
use crate::workspace::Workspace;

pub fn run_app(app: gpui::App, showfile_path: Option<String>) {
    app.with_assets(Assets).run(move |cx: &mut AppContext| {
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
            ShowfileManager::init(showfile_path, cx);

            let view = Workspace::build(cx);
            cx.focus_view(&view);
            view
        });
    });
}
