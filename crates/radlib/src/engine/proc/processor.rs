use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::builtin::{Objects, Patch, Programmer};
use crate::comp::ComponentHandle;
use crate::engine::pipeline::Pipeline;

pub fn start(
    _objects: ComponentHandle<Objects>,
    patch: ComponentHandle<Patch>,
    programmer: ComponentHandle<Programmer>,
    pipeline: Arc<Mutex<Pipeline>>,
) {
    thread::Builder::new()
        .name("processor".to_string())
        .spawn(move || {
            loop {
                // Process objects.
                // FIXME: Implement.

                // Process programmer values.
                programmer.read(|programmer| {
                    let mut pipeline = pipeline.lock().unwrap();
                    for ((fid, attribute), value) in programmer.values() {
                        pipeline.set_value(*fid, attribute.clone(), *value);
                    }
                    drop(pipeline);
                });

                // Resolve values.
                patch.read(|patch| pipeline.lock().unwrap().resolve(patch));

                thread::sleep(Duration::from_secs(10));
            }
        })
        .unwrap();
}
