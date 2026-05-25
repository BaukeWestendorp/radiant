use std::path::PathBuf;

use rd_engine_2::Engine;

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];

    let mut engine = Engine::new(Some(PathBuf::from(path))).unwrap();

    engine.start();
}
