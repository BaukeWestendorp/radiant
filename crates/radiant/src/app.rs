use std::path::PathBuf;

use anyhow::Result;
use backstage::show::Show;
use gpui::{AppContext, WindowOptions};

use crate::workspace::Workspace;

pub fn run_app(app: gpui::App, showfile_path: Option<PathBuf>) -> Result<()> {
    let mut show: Show = match showfile_path {
        Some(showfile_path) => {
            let file = std::fs::File::open(showfile_path.join("show.json"))?;
            serde_json::from_reader(file)?
        }
        None => {
            log::info!("No showfile path provided. Opening a new showfile.");
            Show::default()
        }
    };

    app.run(move |cx: &mut AppContext| {
        smol::block_on(async {
            match show
                .initialize(
                    std::env::var("GDTF_SHARE_USER").unwrap(),
                    std::env::var("GDTF_SHARE_PASSWORD").unwrap(),
                )
                .await
            {
                Ok(_) => {
                    log::info!("Show has been initialized")
                }
                Err(err) => {
                    log::error!("Failed to initialize show: {err}")
                }
            }
        });

        cx.open_window(WindowOptions::default(), |cx| {
            let view = Workspace::build(cx);
            view
        });
    });

    Ok(())
}
