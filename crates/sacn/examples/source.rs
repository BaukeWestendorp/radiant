use std::{thread, time::Duration};

use dmx::{Address, Channel, Multiverse, Universe, UniverseId};
use sacn::source::{Source, SourceConfig};

fn main() {
    let universe_id = UniverseId::new(1).unwrap();
    let mut source = Source::new(SourceConfig {
        name: "Example Source".to_string(),
        ip: "239.255.0.1".parse().unwrap(),
        ..Default::default()
    });

    source.start();

    let mut multiverse = Multiverse::new();
    multiverse.create_universe(Universe::new(universe_id));

    let channel = Channel::new(1).unwrap();
    for i in 0.. {
        let value = dmx::Value(i % u8::MAX);
        let address = Address::new(universe_id, channel);
        multiverse.set_value(&address, value).unwrap();
        source.set_output(multiverse.clone());

        thread::sleep(Duration::from_millis(250));
    }
}
