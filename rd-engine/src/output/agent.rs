use std::sync::RwLock;
use std::thread::{self, JoinHandle};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use thread_priority::{ThreadBuilderExt as _, ThreadPriority};

use crate::dmx::Multiverse;
use crate::output::OutputDefinition;

pub struct OutputAgent {
    scheduler_handle: Option<JoinHandle<()>>,
    scheduler_running: Arc<AtomicBool>,

    notify_tx: flume::Sender<()>,
    notify_rx: flume::Receiver<()>,
    multiverse: Arc<RwLock<Multiverse>>,

    sacn_instances: Vec<super::instance::sacn::SacnInstance>,
    enttec_instances: Vec<super::instance::enttec::EnttecInstance>,
}

impl Default for OutputAgent {
    fn default() -> Self {
        Self::new(OutputDefinition::default()).unwrap()
    }
}

impl OutputAgent {
    pub fn new(definition: OutputDefinition) -> anyhow::Result<Self> {
        let (notify_tx, notify_rx) = flume::bounded(1);
        let multiverse = Arc::new(RwLock::new(Multiverse::new()));

        let sacn_instances = definition
            .sacn
            .instances()
            .iter()
            .map(|instance| super::instance::sacn::SacnInstance::new(instance.clone()))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let enttec_instances = definition
            .enttec
            .instances()
            .iter()
            .map(|instance| super::instance::enttec::EnttecInstance::new(instance.clone()))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Self {
            scheduler_handle: None,
            scheduler_running: Arc::new(AtomicBool::new(false)),

            notify_tx,
            notify_rx,
            multiverse,

            sacn_instances,
            enttec_instances,
        })
    }

    pub(crate) fn start(&mut self) {
        for instance in &mut self.sacn_instances {
            if let Err(err) = instance.start(self.notify_rx.clone(), self.multiverse.clone()) {
                log::error!("Failed to start sACN output instance: {err}");
            }
        }

        for instance in &mut self.enttec_instances {
            if let Err(err) = instance.start(self.notify_rx.clone(), self.multiverse.clone()) {
                log::error!("Failed to start Enttec output instance: {err}");
            }
        }

        self.start_scheduler();
    }

    pub(crate) fn stop(&mut self) {
        self.stop_scheduler();

        for instance in &mut self.sacn_instances {
            instance.stop();
        }

        for instance in &mut self.enttec_instances {
            instance.stop();
        }
    }

    fn start_scheduler(&mut self) {
        const INTERVAL: Duration = Duration::new(0, ((1_000_000_000_f64 / 44.0).round()) as u32);

        if self.scheduler_handle.is_some() {
            log::warn!("Scheduler already running");
            return;
        }

        log::info!("Starting output scheduler");
        let running = self.scheduler_running.clone();
        running.store(true, Ordering::SeqCst);

        let notify_tx = self.notify_tx.clone();
        let handle = thread::Builder::new()
            .name("rd_output_scheduler".to_string())
            .spawn_with_priority(ThreadPriority::Max, move |tp_res| {
                if let Err(err) = tp_res {
                    log::warn!("Failed to set thread priority: {err}");
                };

                let sleeper = spin_sleep::SpinSleeper::default();
                let mut next_tick = Instant::now() + INTERVAL;
                while running.load(Ordering::SeqCst) {
                    let now = Instant::now();
                    if now < next_tick {
                        sleeper.sleep_until(next_tick);
                    } else {
                        let deviation = (now - next_tick).as_secs_f64();
                        if now > next_tick + INTERVAL {
                            // If we are more than one tick late, skip ahead to catch up.
                            let ticks_missed =
                                (deviation / INTERVAL.as_secs_f64()).floor() as u32 + 1;
                            next_tick += INTERVAL * ticks_missed;
                        }
                    }

                    let _ = notify_tx.try_send(());

                    next_tick += INTERVAL;
                }
            })
            .expect("Failed to spawn scheduler thread");

        self.scheduler_handle = Some(handle);
    }

    fn stop_scheduler(&mut self) {
        if self.scheduler_handle.is_some() {
            log::info!("Stopping output scheduler");
            self.scheduler_running.store(false, Ordering::SeqCst);
            if let Some(handle) = self.scheduler_handle.take() {
                let _ = handle.join();
            }
        }
    }

    pub(crate) fn update(&self, multiverse: Multiverse) {
        *self.multiverse.write().unwrap() = multiverse;
    }
}

impl Drop for OutputAgent {
    fn drop(&mut self) {
        self.stop_scheduler();
    }
}
