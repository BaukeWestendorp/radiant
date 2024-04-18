use std::path::PathBuf;

use backstage::show::{AttributeValue, FixtureId};
use gpui::{AppContext, Global, WindowOptions};

use crate::{
    output::{artnet::ArtnetDmxProtocol, OutputManager},
    showfile::Showfile,
    workspace::Workspace,
};

pub fn run_app(app: gpui::App, showfile_path: Option<PathBuf>) {
    app.run(move |cx: &mut AppContext| {
        Showfile::init(showfile_path, cx)
            .map_err(|err| log::error!("Failed to initialize showfile: {err}"))
            .ok();

        OutputManager::init(cx);
        OutputManager::register_protocol(ArtnetDmxProtocol::new("0.0.0.0", 0, 0).unwrap(), cx);

        Showfile::update(cx, |showfile, _cx| {
            let fixture = showfile
                .show
                .patchlist()
                .fixture(&FixtureId::new(101))
                .unwrap()
                .clone();
            showfile
                .show
                .programmer_mut()
                .set_attribute(&fixture, "Dimmer".to_string(), AttributeValue::new(0.5))
                .unwrap();
        });

        cx.open_window(WindowOptions::default(), |cx| {
            let view = Workspace::build(cx);
            view
        });
    });
}
