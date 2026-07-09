use std::{thread, time::Duration};

use rd_midi::MidiPacket;
use rd_service::Service;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let mut service = Service::new(MidiService, rd_midi::MidiInputServiceRunner::new()?);
    service.start()?;

    thread::sleep(Duration::from_secs(3600));

    Ok(())
}

struct MidiService;

impl rd_service::Delegate for MidiService {
    type Error = anyhow::Error;

    type Data = MidiPacket;

    fn on_start(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_frame(&self, packet: MidiPacket) -> Result<(), Self::Error> {
        eprintln!("{:?}", packet);
        Ok(())
    }

    fn on_stop(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}
