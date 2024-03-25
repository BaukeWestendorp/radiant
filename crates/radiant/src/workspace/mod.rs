use anyhow::Result;
use backstage::show::Show;
use backstage::showfile::Showfile;
use gdtf_share::GdtfShare;
use gpui::{AppContext, AsyncAppContext, Context, Model, Task, Timer};

use std::env;
use std::fs::File;
use std::time::Duration;

use screen::Screen;

pub mod screen;

pub mod action {
    use gpui::actions;

    actions!(workspace, [Debug]);
}

const DMX_OUTPUT_RATE: Duration = Duration::from_millis(1000 / 40);

pub struct Workspace {
    dmx_output_loop_task: Option<Task<()>>,

    show: Model<Show>,
}

impl Workspace {
    pub fn new(cx: &mut AsyncAppContext) -> Task<Result<Self>> {
        cx.spawn(move |mut cx| async move {
            let show = get_show().await?;
            let show_model = cx.new_model(|_cx| show)?;

            let _main_screen = cx.update(|cx| Screen::open_window(show_model.clone(), cx))?;

            Ok(Self {
                show: show_model,
                dmx_output_loop_task: None,
            })
        })
    }

    pub fn start_dmx_output_loop(&mut self, cx: &mut AppContext) {
        self.dmx_output_loop_task = Some(cx.spawn({
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
        }))
    }

    pub fn stop_dmx_output_loop(&mut self) {
        self.dmx_output_loop_task = None;
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
