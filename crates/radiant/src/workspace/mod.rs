use std::{io, path::PathBuf};

use gpui::*;

use crate::{dmx_io::DmxIo, showfile::Showfile};

pub mod frame;
pub mod main_window;

pub use main_window::*;

pub struct Workspace {
    #[allow(unused)]
    main_window: WindowHandle<MainWindow>,
}

impl Workspace {
    pub fn new(showfile_path: Option<PathBuf>, cx: &mut AppContext) -> anyhow::Result<Self> {
        init_showfile(showfile_path, cx)?;
        init_dmx_io(cx)?;

        cx.set_global(DmxIo::new());

        let main_window = open_main_window(cx)?;

        Ok(Self { main_window })
    }
}

fn init_showfile(showfile_path: Option<PathBuf>, cx: &mut AppContext) -> io::Result<()> {
    let showfile = match showfile_path {
        Some(path) => {
            let file = std::fs::File::open(path)?;
            serde_json::from_reader(file)?
        }
        None => Showfile::default(),
    };

    cx.set_global(showfile);

    Ok(())
}

fn init_dmx_io(cx: &mut AppContext) -> anyhow::Result<()> {
    let mut dmx_io = DmxIo::new();

    for artnet_protocol in Showfile::global(cx).dmx_protocols().artnet() {
        dmx_io.add_artnet_node(artnet_protocol.clone())?;
    }

    dmx_io.start_emitting(cx);

    cx.set_global(dmx_io);

    Ok(())
}

fn open_main_window(cx: &mut AppContext) -> anyhow::Result<WindowHandle<MainWindow>> {
    cx.open_window(window_options(cx), |cx| MainWindow::build(cx))
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
        window_min_size: Some(size(px(600.0), px(400.0))),
        ..Default::default()
    }
}
