use std::net::ToSocketAddrs;

use dmx::{Channel, Universe, UniverseId};
use sacn::source::{Source, SourceConfig};

fn main() {
    let mut source = Source::new(SourceConfig {
        name: "Example Source".to_string(),
        addr: "192.168.2.5:5568".to_socket_addrs().unwrap().next().unwrap(),
        preview_data: false,
        priority: 100,
        sync_addr: 0,
        force_synchronization: false,
        universe: UniverseId::new(1).unwrap(),
    });

    let mut universe = Universe::new(UniverseId::new(1).unwrap());
    universe.set_value(&Channel::new(1).unwrap(), dmx::Value(42));

    source.set_output(universe);
    source.start().unwrap();
}
