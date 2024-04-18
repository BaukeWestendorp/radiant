use std::time::Duration;

use gpui::AppContext;

use crate::showfile::Showfile;

pub const DMX_UPDATE_RATE: Duration = Duration::from_millis(1000 / 40);

pub fn start_dmx_output_loop(cx: &mut AppContext) {
    cx.spawn(|cx| async move {
        loop {
            cx.read_global::<Showfile, _>(|showfile, _cx| {
                let dmx_output = showfile.show.get_dmx_output();
                dbg!(dmx_output);
            })
            .unwrap();

            cx.background_executor().timer(DMX_UPDATE_RATE).await;
        }
    })
    .detach();
}
