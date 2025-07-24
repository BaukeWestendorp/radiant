use std::sync::Arc;
use std::thread;

use crate::show::Show;

pub fn start(_show: Arc<Show>) {
    thread::Builder::new()
        .name("control_surface_handler".to_string())
        .spawn(move || loop {})
        .unwrap();
}
