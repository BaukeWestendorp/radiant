use dmx::{Address, Channel, Multiverse, Universe, UniverseId};
use sacn::source::{Source, SourceConfig};
use std::{thread, time::Duration};

fn main() {
    let universe_id = UniverseId::new(1).unwrap();
    let mut multiverse = Multiverse::new();
    multiverse.create_universe(universe_id, Universe::new());

    // Create the source.
    let mut source = Source::new(SourceConfig {
        name: "Example Source".to_string(),
        ip: "239.255.0.1".parse().unwrap(),
        ..Default::default()
    })
    .unwrap();

    // Start the source updater thread.
    source.start();

    let channel = Channel::new(1).unwrap();
    for i in 0.. {
        // Update a channel in the data.
        let value = dmx::Value(i % u8::MAX);
        let address = Address::new(universe_id, channel);
        multiverse.set_value(&address, value).unwrap();

        // Set the output for the source to send over the socket.
        source.set_output(multiverse.clone());

        // Wait 250ms before updating the data.
        thread::sleep(Duration::from_millis(250));
    }
}
