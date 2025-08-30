use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::attr::AttributeValue;
use crate::builtin::{FixtureId, Patch};
use crate::comp::ComponentHandle;
use crate::engine::pipeline::Pipeline;

pub fn start(patch: ComponentHandle<Patch>, pipeline: Arc<Mutex<Pipeline>>) {
    thread::Builder::new()
        .name("processor".to_string())
        .spawn(move || {
            loop {
                pipeline.lock().unwrap().set_value(
                    FixtureId(NonZeroU32::new(101).unwrap()),
                    crate::attr::Attribute::Dimmer,
                    AttributeValue::new(0.75),
                );
                patch.read(|patch| pipeline.lock().unwrap().resolve(&patch));
                thread::sleep(Duration::from_secs(10));
            }
        })
        .unwrap();
}
