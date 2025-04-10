use sacn::source::{Source, SourceConfig};
use std::{sync::Arc, thread};

fn main() {
    // Create the source.
    let source = SourceConfig { name: "Example Source".to_string(), ..Default::default() };
    let source = Arc::new(Source::new(source).unwrap());

    source.set_universe(1, vec![0u8; 512]);

    // Start the source thread.
    thread::spawn({
        let source = Arc::clone(&source);
        move || {
            source.start().unwrap();
        }
    });

    for ix in 0.. {
        let mut data = vec![0u8; 512];
        // Create a wave pattern
        for i in 0..512 {
            let wave1 = ((i as f32 * 0.1 + ix as f32 * 0.05).sin() * 127.0 + 127.0) as u8;
            let wave2 = ((i as f32 * 0.2 - ix as f32 * 0.03).cos() * 127.0 + 127.0) as u8;
            let wave3 = ((i as f32 * 0.15 + ix as f32 * 0.07).sin() * 127.0 + 127.0) as u8;
            data[i] = ((wave1 as u16 + wave2 as u16 + wave3 as u16) / 3) as u8;
        }
        source.set_universe(1, data);

        spin_sleep::sleep(std::time::Duration::from_millis(250));
    }
}
