use std::path::PathBuf;

use gpui::*;
use main_window::MainWindow;
use show::{DmxProtocols, Show};

use crate::dmx_io::DmxIo;

pub mod frame;
pub mod main_window;

pub struct Workspace {
    #[allow(unused)]
    show: Model<Show>,
    #[allow(unused)]
    dmx_io: DmxIo,
}

impl Workspace {
    pub fn new(showfile_path: Option<PathBuf>, cx: &mut AppContext) -> anyhow::Result<Self> {
        let show = init_show(showfile_path, cx)?;
        let show_model = cx.new_model(move |_| show);

        let dmx_protocols = show_model.read(cx).dmx_protocols.clone();
        let dmx_io = init_dmx_io(cx, &dmx_protocols)?;

        open_windows(show_model.clone(), cx)?;

        Ok(Self {
            show: show_model,
            dmx_io,
        })
    }
}

fn init_show(showfile_path: Option<PathBuf>, cx: &mut AppContext) -> anyhow::Result<Show> {
    Ok(match showfile_path {
        Some(path) => Show::try_read(&path, cx)?,
        None => Show::new(cx),
    })
}

fn init_dmx_io(cx: &mut AppContext, dmx_protocols: &Model<DmxProtocols>) -> anyhow::Result<DmxIo> {
    let mut dmx_io = DmxIo::new();

    for artnet_protocol in dmx_protocols.read(cx).artnet() {
        dmx_io.add_artnet_node(artnet_protocol.clone())?;
    }

    dmx_io.start_emitting(cx);

    Ok(dmx_io)
}

fn open_windows(show: Model<Show>, cx: &mut AppContext) -> anyhow::Result<()> {
    cx.open_window(window_options(cx), |cx| MainWindow::build(show, cx))?;
    Ok(())
}

fn window_options(cx: &AppContext) -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
            None,
            size(px(1280.0), px(800.0)),
            cx,
        ))),
        titlebar: Some(TitlebarOptions {
            title: Some("Radiant".into()),
            ..Default::default()
        }),
        window_min_size: Some(size(px(640.0), px(400.0))),
        ..Default::default()
    }
}
