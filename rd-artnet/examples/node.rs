use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
    time::Duration,
};

use rd_artnet::{FixedString, PortAddress, PortConfig, PortDirection, Universe};

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

                    t.fetch_add(1, Ordering::Relaxed);

                    Ok(())
                }
            }),
        )
        .with_port(PortConfig::new(PortDirection::Output(PortAddress::from_absolute(1).unwrap()))),
    )?;

    thread::sleep(Duration::from_secs(3600));

    Ok(())
}
