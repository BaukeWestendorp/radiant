use std::{
    sync::RwLock,
    thread,
    time::{Duration, Instant},
};

use rd_artnet::{FixedString, Universe};

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_builder().filter_level(log::LevelFilter::Info).init();

    let mut service = rd_service::Service::new(
        ArtNetOutput::new(),
        rd_service::Scheduled::new(Duration::from_secs_f64(1.0 / 40.0)),
    );

    service.start()?;

    thread::sleep(Duration::from_secs(3600));

    Ok(())
}

struct ArtNetOutput {
    node: RwLock<Option<rd_artnet::Node>>,
}

impl ArtNetOutput {
    fn new() -> Self {
        Self { node: RwLock::new(None) }
    }
}

impl rd_service::Delegate for ArtNetOutput {
    type Error = anyhow::Error;

    type Data = Instant;

    fn on_start(&self) -> Result<(), Self::Error> {
        *self.node.write().unwrap() = Some(rd_artnet::Node::new(rd_artnet::NodeConfig {
            name: FixedString::try_from_str("rd-artnet")?,
            ..Default::default()
        })?);

        Ok(())
    }

    fn on_frame(&self, _instant: Instant) -> Result<(), Self::Error> {
        let mut universe = Universe::new();
        for ix in 0..512 {
            universe.set_channel(ix, ix as u8)?;
        }

        self.node.read().unwrap().as_ref().unwrap().send_dmx(universe)?;

        Ok(())
    }

    fn on_stop(&self) -> Result<(), Self::Error> {
        let _ = self.node.write().unwrap().take();

        Ok(())
    }
}
