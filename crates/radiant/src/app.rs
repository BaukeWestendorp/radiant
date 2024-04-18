use std::{path::PathBuf, time::Duration};

use gpui::{AppContext, WindowOptions};

use crate::{showfile::Showfile, workspace::Workspace};

pub const DMX_UPDATE_RATE: Duration = Duration::from_millis(1000 / 40);

pub fn run_app(app: gpui::App, showfile_path: Option<PathBuf>) {
    app.run(move |cx: &mut AppContext| {
        Showfile::init(showfile_path, cx)
            .map_err(|err| log::error!("Failed to initialize showfile: {err}"))
            .ok();

        cx.spawn(|cx| async move {
            loop {
                cx.read_global::<Showfile, _>(|showfile, _cx| {
                    let dmx_output = showfile.show.get_dmx_output();
                })
                .unwrap();

                cx.background_executor().timer(DMX_UPDATE_RATE).await;
            }
        })
        .detach();

        cx.open_window(WindowOptions::default(), |cx| {
            let view = Workspace::build(cx);
            view
        });
    });
}
