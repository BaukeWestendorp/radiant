use anyhow::Result;
use backstage::show::Show;
use gpui::{AsyncAppContext, Context, Model, Task, Timer};

use std::fs::File;
use std::time::Duration;

use crate::layout::screen::Screen;
use crate::layout::window_grid::{GridBounds, GridPoint, GridSize};
use crate::layout::{PoolWindow, PoolWindowKind, Window, WindowGrid, WindowKind};

pub mod action {
    use backstage::command::Command;
    use gpui::impl_actions;

    impl_actions!(workspace, [ExecuteCommand, SetCurrentCommand]);

    #[derive(Clone, PartialEq, serde::Deserialize)]
    pub struct ExecuteCommand(pub Command);

    #[derive(Clone, PartialEq, serde::Deserialize)]
    pub struct SetCurrentCommand(pub Option<Command>);
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
                    bounds: GridBounds::new(GridPoint::new(5, 0), GridSize::new(3, 3)),
                    kind: WindowKind::Pool(PoolWindow {
                        kind: PoolWindowKind::Group,
                        scroll_offset: 0,
                    }),
                });
                window_grid.add_window(Window {
                    bounds: GridBounds::new(GridPoint::new(8, 0), GridSize::new(3, 3)),
                    kind: WindowKind::Pool(PoolWindow {
                        kind: PoolWindowKind::ColorPreset,
                        scroll_offset: 0,
                    }),
                });
                window_grid.add_window(Window {
                    bounds: GridBounds::new(GridPoint::new(0, 5), GridSize::new(10, 3)),
                    kind: WindowKind::FixtureSheet,
                });
                window_grid
            })?;

            cx.update(|cx| {
                Screen::open_window(show_model.clone(), window_grid, cx);
            })?;

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
                            show.recalculate_stage_output();
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
    Show::from_file(file).await
}
