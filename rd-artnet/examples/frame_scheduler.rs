use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
    time::Duration,
};

use rd_artnet::{FixedString, FrameScheduler, PortAddress, PortConfig, PortDirection, Universe};

const REFRESH_RATE: u16 = 5;

fn main() -> rd_artnet::Result<()> {
    pretty_env_logger::formatted_builder().filter_level(log::LevelFilter::Info).init();

    let _node = rd_artnet::Node::new(
        rd_artnet::NodeConfig::new(
            FixedString::try_from_str("rd-artnet")?,
            FixedString::try_from_str("rd-artnet")?,
            Arc::new({
                let t = AtomicUsize::new(0);
                move |universe: &mut Universe, _port_address: PortAddress| {
                    let t_value = t.load(Ordering::Relaxed);
                    for ix in 0..512 {
                        universe.set_channel(ix, ((ix + t_value) % u8::MAX as usize) as u8)?;
                    }

                    t.fetch_add(4, Ordering::Relaxed);

                    Ok(())
                }
            }),
        )
        .with_port(PortConfig::new(PortDirection::Output(PortAddress::from_raw(1).unwrap())))
        .with_frame_scheduler(FrameScheduler::External {
            refresh_rate: REFRESH_RATE,
            notifier: Arc::new(Box::new(|notify_tx| {
                // NOTE: This is a very bad and innacurate way to make a frame scheduler,
                //       but for this example it will suffice.
                loop {
                    let _ = notify_tx.send(());
                    thread::sleep(Duration::from_secs_f64(1.0 / REFRESH_RATE as f64));
                }
            })),
        }),
    )?;

    thread::sleep(Duration::from_secs(3600));

    Ok(())
}
