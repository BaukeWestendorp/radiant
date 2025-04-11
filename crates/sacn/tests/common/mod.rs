use sacn::{
    Universe,
    receiver::{Receiver, ReceiverConfig},
    source::{Source, SourceConfig},
};
use std::{sync::Arc, time::Duration};

const TEST_SOURCE_IP: &str = "127.0.0.1";
const TEST_RECV_IP: &str = "0.0.0.0";
const TEST_PORT: u16 = 25453;

pub fn start_test_source_thread() -> Arc<Source> {
    // Create the source.
    let config =
        SourceConfig { ip: TEST_SOURCE_IP.parse().unwrap(), port: TEST_PORT, ..Default::default() };
    let source = Arc::new(Source::new(config).unwrap());

    // Start the source thread.
    std::thread::spawn({
        let source = Arc::clone(&source);
        move || {
            source.start().unwrap();
        }
    });

    source
}

pub fn start_test_receiver() -> Receiver {
    let config =
        ReceiverConfig { ip: TEST_RECV_IP.parse().unwrap(), port: TEST_PORT, ..Default::default() };
    Receiver::start(config).unwrap()
}

pub fn recv(receiver: &Receiver) -> Universe {
    match receiver.recv() {
        Ok(recv_universe) => return recv_universe,
        Err(err) => panic!("Failed to receive universe: {}", err),
    }
}

pub fn recv_until_all(receiver: &Receiver, mut universes: Vec<Universe>) {
    while !universes.is_empty() {
        let received = receiver.recv();
        match received {
            Ok(universe) => {
                for expected in universes.clone() {
                    if universe == expected {
                        universes.retain(|u| *u != expected);
                        break;
                    }
                }
            }
            Err(err) => panic!("Failed to receive universe: {}", err),
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

#[macro_export]
macro_rules! create_test_universe {
    () => {
        create_test_universe!(42)
    };
    ($number:expr) => {{
        let mut universe = Universe::new($number);
        universe.start_code_slot = 25;
        for i in 0..universe.data_slots.capacity() {
            universe.data_slots.push(i as sacn::Slot);
        }
        universe
    }};
}

pub use create_test_universe;
