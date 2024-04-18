use anyhow::Result;
use gpui::{AppContext, Global};
use std::path::PathBuf;

use backstage::show::Show;

pub struct Showfile {
    pub show: Show,
}

impl Showfile {
    pub fn init(showfile_path: Option<PathBuf>, cx: &mut AppContext) -> Result<()> {
        let mut show: Show = match showfile_path {
            Some(showfile_path) => {
                let file = std::fs::File::open(showfile_path.join("show.json"))?;
                serde_json::from_reader(file)?
            }
            None => {
                log::info!("No showfile path provided. Opening an empty showfile.");
                Show::default()
            }
        };

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

        cx.set_global(Showfile { show });

        Ok(())
    }
}

impl Global for Showfile {}
