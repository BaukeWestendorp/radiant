use sacn::receiver::{Receiver, ReceiverConfig};

fn main() {
    let receiver = Receiver::start(ReceiverConfig::default()).unwrap();

    loop {
        if let Ok((id, universe)) = receiver.recv() {
            println!("Universe: {id}: {universe:?}");
        }
    }
}
