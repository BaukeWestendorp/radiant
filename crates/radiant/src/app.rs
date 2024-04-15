use gpui::{AppContext, WindowOptions};

use crate::workspace::Workspace;

pub fn run_app(app: gpui::App, showfile_path: Option<String>) {
    app.run(move |cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            let view = Workspace::build(cx);
            view
        });
    })
}
