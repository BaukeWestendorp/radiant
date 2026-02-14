use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use spin_sleep::SpinSleeper;
use zeevonk::Zeevonk;

use crate::{
    compositor::{Composition, Compositor},
    object::ObjectRegistry,
    programmer::Programmer,
};

const FRAME_TIME: f64 = 1.0 / 60.0;

pub struct OutputAgent {
    objects: Arc<ObjectRegistry>,
    programmer: Arc<Programmer>,
    zeevonk: Arc<Zeevonk>,
}

impl OutputAgent {
    pub fn new(
        objects: Arc<ObjectRegistry>,
        programmer: Arc<Programmer>,
        zeevonk: Arc<Zeevonk>,
    ) -> Self {
        Self { objects, programmer, zeevonk }
    }

    pub fn start(&self) {
        let _ = thread::Builder::new().name("rd_output_agent".to_string()).spawn({
            let objects = Arc::clone(&self.objects);
            let programmer = Arc::clone(&self.programmer);
            let zeevonk = Arc::clone(&self.zeevonk);
            move || {
                let sleeper = SpinSleeper::default();
                let frame_duration = Duration::from_secs_f64(FRAME_TIME);
                loop {
                    let deadline = Instant::now() + frame_duration;

                    let compositor = Compositor::new(Arc::clone(&programmer), Arc::clone(&objects));
                    let Composition { attribute_values, highlighted_fixtures } =
                        compositor.compose();
                    zeevonk.clear_attribute_values();
                    zeevonk.set_attribute_values(attribute_values);
                    zeevonk.clear_highlighted_fixtures();
                    zeevonk.set_highlighted_fixtures(highlighted_fixtures.to_owned());

                    sleeper.sleep_until(deadline);
                }
            }
        });
    }
}
