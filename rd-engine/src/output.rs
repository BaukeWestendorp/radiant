use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

use spin_sleep::SpinSleeper;
use thread_priority::{ThreadBuilderExt as _, ThreadPriority};
use zeevonk::{Zeevonk, project::FixtureId};

use crate::{Composition, Compositor, EffectAgent, ObjectRegistry, Programmer};

const FRAME_TIME: f64 = 1.0 / 60.0;

pub struct OutputAgent {
    objects: Arc<ObjectRegistry>,
    programmer: Arc<Programmer>,
    effect_agent: Arc<EffectAgent>,
    zeevonk: Arc<Zeevonk>,
    selection: Arc<RwLock<Vec<FixtureId>>>,
    highlight: Arc<RwLock<bool>>,
}

impl OutputAgent {
    pub fn new(
        objects: Arc<ObjectRegistry>,
        programmer: Arc<Programmer>,
        effect_agent: Arc<EffectAgent>,
        zeevonk: Arc<Zeevonk>,
        selection: Arc<RwLock<Vec<FixtureId>>>,
        highlight: Arc<RwLock<bool>>,
    ) -> Self {
        Self { objects, programmer, effect_agent, zeevonk, selection, highlight }
    }

    pub fn start(&self) {
        let _ = thread::Builder::new().name("rd_output_agent".to_string()).spawn_with_priority(
            ThreadPriority::Max,
            {
                let objects = Arc::clone(&self.objects);
                let programmer = Arc::clone(&self.programmer);
                let effect_agent = Arc::clone(&self.effect_agent);
                let zeevonk = Arc::clone(&self.zeevonk);
                let selection = Arc::clone(&self.selection);
                let highlight = Arc::clone(&self.highlight);
                move |_| {
                    let sleeper = SpinSleeper::default();
                    let frame_duration = Duration::from_secs_f64(FRAME_TIME);
                    loop {
                        let deadline = Instant::now() + frame_duration;

                        let mut compositor = Compositor::new(
                            Arc::clone(&objects),
                            Arc::clone(&programmer),
                            Arc::clone(&effect_agent),
                        );

                        if *highlight.read().unwrap() {
                            compositor.highlight_fixtures(selection.read().unwrap().clone());
                        }

                        let Composition { attribute_values, highlighted_fixtures } =
                            match compositor.compose() {
                                Ok(comp) => comp,
                                Err(err) => {
                                    log::error!("error while composing: {:?}", err);
                                    continue;
                                }
                            };

                        zeevonk.clear_attribute_values();
                        zeevonk.set_attribute_values(attribute_values);
                        zeevonk.clear_highlighted_fixtures();
                        zeevonk.set_highlighted_fixtures(highlighted_fixtures.to_vec());

                        sleeper.sleep_until(deadline);
                    }
                }
            },
        );
    }
}
