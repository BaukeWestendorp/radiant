use assets::Assets;
use dmx::{DmxChannel, DmxOutput};
use gpui::{AppContext, WindowOptions};

use crate::output::{ArtnetDmxProtocol, DmxOutputManager};
use crate::workspace::Workspace;

pub fn run_app(app: gpui::App) {
    app.with_assets(Assets).run(move |cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            DmxOutputManager::init(cx);

            // Initialize temporary test state for the app.
            DmxOutputManager::register_protocol(ArtnetDmxProtocol::new("0.0.0.0").unwrap(), cx);
            let mut dmx_output = DmxOutput::new();
            dmx_output
                .set_channel(&DmxChannel::new(0, 0).unwrap(), 127)
                .unwrap();
            DmxOutputManager::set_dmx_output(dmx_output, cx);

            Workspace::build(cx)
        });
    });
}
