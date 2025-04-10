use sacn::receiver::{Receiver, ReceiverConfig};
use std::{thread, time::Duration};

fn main() {
    // Create the receiver.
    let mut receiver =
        Receiver::new(ReceiverConfig { ip: "127.0.0.1".parse().unwrap(), ..Default::default() });

    // Start the receiver.
    receiver.start().unwrap();

    loop {
        let data = receiver.data();

        // Wait 250ms before updating the data.
        thread::sleep(Duration::from_millis(250));
    }
}
