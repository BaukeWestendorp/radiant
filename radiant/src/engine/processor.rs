use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::pipeline::Pipeline;
use crate::show::Show;

const PROCESSOR_FRAME_TIME: Duration = Duration::from_millis(30);

pub fn start(pipeline: Arc<Mutex<Pipeline>>, show: Arc<Show>) {
    thread::Builder::new()
        .name("processor".to_string())
        .spawn(move || {
            loop {
                // Put each fixture's default values into the output pipeline before resolving
                // other values.
                for fixture in show.patch().fixtures().to_vec() {
                    for (attribute, value) in fixture.get_default_attribute_values(show.patch()) {
                        pipeline.lock().unwrap().set_value(fixture.fid(), attribute.clone(), value);
                    }
                }

                // TODO: Resolve and merge executor outputs.

                // Merge programmer values into output pipeline.
                for (fid, attr, value) in show.programmer().values() {
                    pipeline.lock().unwrap().set_value(fid, attr.clone(), value);
                }

                // Resolve output pipeline.
                pipeline.lock().unwrap().resolve(&show.patch());

                spin_sleep::sleep(PROCESSOR_FRAME_TIME);
            }
        })
        .unwrap();
}
