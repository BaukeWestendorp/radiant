use std::thread;
use std::time::Duration;

pub fn start() {
    thread::Builder::new()
        .name("protocols".to_string())
        .spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(10));
            }
        })
        .unwrap();
}
