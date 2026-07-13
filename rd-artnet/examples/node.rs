use std::{thread, time::Duration};

use rd_artnet::{FixedString, Universe};

fn main() -> rd_artnet::Result<()> {
    pretty_env_logger::formatted_builder().filter_level(log::LevelFilter::Info).init();

    let handle = thread::spawn(|| -> rd_artnet::Result<()> {
        let node = rd_artnet::Node::new(rd_artnet::NodeConfig {
            name: FixedString::try_from_str("rd-artnet")?,
            ..Default::default()
        })?;

        loop {
            let mut universe = Universe::new();
            for ix in 0..512 {
                universe.set_channel(ix, ix as u8)?;
            }
            node.send_dmx(universe)?;

            // NOTE: This is a very inaccurate way to do an interval loop,
            //       but for this example it's fine.
            thread::sleep(Duration::from_secs_f64(1.0 / 40.0));
        }
    });

    thread::sleep(Duration::from_secs(3600));

    handle.join().unwrap()?;

    Ok(())
}
