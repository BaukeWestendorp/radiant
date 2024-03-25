use anyhow::Result;
use backstage::show::Show;
use backstage::showfile::Showfile;
use gdtf_share::GdtfShare;
use gpui::{AsyncAppContext, Context, Model, Task, Timer};

use std::env;
use std::fs::File;
use std::time::Duration;

use self::layout::screen::Screen;
use self::layout::window_grid::{GridBounds, GridPoint, GridSize};
use self::layout::{PoolWindow, Window, WindowGrid, WindowKind};

pub mod layout;

pub mod action {
    use gpui::actions;

    actions!(workspace, [Debug]);
}

const DMX_OUTPUT_RATE: Duration = Duration::from_millis(1000 / 40);

pub struct Workspace {
    show: Model<Show>,
}

impl Workspace {
    pub fn new(cx: &AsyncAppContext) -> Task<Result<Self>> {
        cx.spawn(move |mut cx| async move {
            let show = get_show().await?;
            let show_model = cx.new_model(|_cx| show)?;

            let window_grid = cx.new_model(|_cx| {
                let mut window_grid = WindowGrid::new();
                window_grid.add_window(Window {
                    bounds: GridBounds::new(GridPoint::new(0, 0), GridSize::new(5, 5)),
                    kind: WindowKind::Executors,
                });
                window_grid.add_window(Window {
                    bounds: GridBounds::new(GridPoint::new(5, 0), GridSize::new(5, 5)),
                    kind: WindowKind::Pool(PoolWindow {
                        kind: layout::PoolWindowKind::Group,
                        scroll_offset: 0,
                    }),
                });
                window_grid.add_window(Window {
                    bounds: GridBounds::new(GridPoint::new(0, 5), GridSize::new(5, 5)),
                    kind: WindowKind::Pool(PoolWindow {
                        kind: layout::PoolWindowKind::Color,
                        scroll_offset: 0,
                    }),
                });
                window_grid
            })?;
            let _main_screen =
                cx.update(|cx| Screen::open_window(show_model.clone(), window_grid, cx))?;

            Ok(Self { show: show_model })
        })
    }

    pub fn start_dmx_output_loop(&mut self, cx: &AsyncAppContext) -> Task<()> {
        cx.spawn({
            let show = self.show.clone();
            |cx| async move {
                loop {
                    cx.update(|cx| {
                        log::trace!("Outputting DMX data...");
                        show.update(cx, |show, _cx| {
                            show.send_stage_output_to_dmx_protocols();
                        });
                    })
                    .unwrap();

                    Timer::after(DMX_OUTPUT_RATE).await;
                }
            }
        })
    }
}

async fn get_show() -> Result<Show> {
    let file = File::open("show.json")?;
    let showfile = Showfile::from_file(file)?;
    let user = env::var("GDTF_SHARE_API_USER")?;
    let password = env::var("GDTF_SHARE_API_PASSWORD")?;
    let gdtf_share = GdtfShare::auth(user, password).await?;
    Ok(Show::new(showfile, gdtf_share).await)
}
