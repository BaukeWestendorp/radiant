use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use anyhow::Context;
use thread_priority::{ThreadBuilderExt, ThreadPriority};

pub trait ServiceDelegate: Send + Sync {
    fn on_start(&self) -> anyhow::Result<()>;

    fn on_tick(&self) -> anyhow::Result<()>;

    fn on_stop(&self) -> anyhow::Result<()>;

    fn name(&self) -> &'static str;
}

pub struct Service<D: ServiceDelegate> {
    delegate: Arc<D>,

    runner: Runner,
    running: Arc<AtomicBool>,

    ticker_handle: Option<JoinHandle<()>>,
}

impl<D: ServiceDelegate + 'static> Service<D> {
    pub fn new_scheduled(delegate: D, interval: Duration) -> Self {
        Self {
            delegate: Arc::new(delegate),
            runner: Runner::Scheduled { interval, scheduler_handle: None },
            running: Arc::new(AtomicBool::new(false)),
            ticker_handle: None,
        }
    }

    pub fn new_driven(delegate: D, notify_rx: flume::Receiver<()>) -> Self {
        Self {
            delegate: Arc::new(delegate),
            runner: Runner::Driven { notify_rx, driver_handle: None },
            running: Arc::new(AtomicBool::new(false)),
            ticker_handle: None,
        }
    }

    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        if self.running() {
            log::warn!("{} service is already running", self.delegate.name());
            return Ok(());
        };

        let (tick_tx, tick_rx) = flume::bounded(1);

        self.runner.start(tick_tx, Arc::clone(&self.running), self.delegate.as_ref());
        self.delegate
            .on_start()
            .with_context(|| format!("Failed to start {} service", self.delegate.name()))?;
        self.running.store(true, Ordering::SeqCst);

        let delegate = Arc::clone(&self.delegate);
        let handle = thread::spawn(move || {
            loop {
                match tick_rx.recv() {
                    Ok(()) => {
                        if let Err(err) = delegate.on_tick() {
                            log::error!(
                                "An error occured in the {} service: {err}",
                                delegate.name()
                            );
                        }
                    }
                    Err(flume::RecvError::Disconnected) => break,
                }
            }
        });
        self.ticker_handle = Some(handle);

        Ok(())
    }

    pub fn stop(&mut self) -> anyhow::Result<()> {
        self.running.store(false, Ordering::SeqCst);
        self.delegate
            .on_stop()
            .with_context(|| format!("Failed to stop {} service", self.delegate.name()))?;
        self.runner.stop();

        if let Some(handle) = self.ticker_handle.take() {
            let _ = handle.join();
        }

        Ok(())
    }

    pub fn running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

enum Runner {
    Scheduled { interval: Duration, scheduler_handle: Option<JoinHandle<()>> },
    Driven { notify_rx: flume::Receiver<()>, driver_handle: Option<JoinHandle<()>> },
}

impl Runner {
    pub fn start<D: ServiceDelegate>(
        &mut self,
        tick_tx: flume::Sender<()>,
        running: Arc<AtomicBool>,
        delegate: &D,
    ) {
        running.store(true, Ordering::SeqCst);

        match self {
            Runner::Scheduled { interval, scheduler_handle } => {
                let interval = *interval;
                let running = running.clone();
                let handle = thread::Builder::new()
                    .name(format!("rd_{}_service", delegate.name().to_lowercase()))
                    .spawn_with_priority(ThreadPriority::Max, move |tp_res| {
                        if let Err(err) = tp_res {
                            log::warn!("Failed to set thread priority: {err}");
                        };

                        let sleeper = spin_sleep::SpinSleeper::default();
                        let mut next_tick = Instant::now() + interval;
                        while running.load(Ordering::SeqCst) {
                            let now = Instant::now();
                            if now < next_tick {
                                sleeper.sleep_until(next_tick);
                            } else {
                                let deviation = (now - next_tick).as_secs_f64();
                                if now > next_tick + interval {
                                    // We need to add this prefix byte to convert the buffer's 0-index to a 1-index.
                                    let ticks_missed =
                                        (deviation / interval.as_secs_f64()).floor() as u32 + 1;
                                    next_tick += interval * ticks_missed;
                                }
                            }

                            let _ = tick_tx.try_send(());

                            next_tick += interval;
                        }
                    })
                    .expect("Failed to spawn scheduler thread");

                *scheduler_handle = Some(handle);
            }
            Runner::Driven { notify_rx, driver_handle } => {
                let notify_rx = notify_rx.clone();
                let running = running.clone();
                let handle = thread::Builder::new()
                    .name(format!("rd_{}_driven", delegate.name().to_lowercase()))
                    .spawn_with_priority(ThreadPriority::Max, move |tp_res| {
                        if let Err(err) = tp_res {
                            log::warn!("Failed to set thread priority: {err}");
                        };

                        while running.load(Ordering::SeqCst) {
                            match notify_rx.recv_timeout(Duration::from_millis(50)) {
                                Ok(()) => {
                                    let _ = tick_tx.try_send(());
                                }
                                Err(flume::RecvTimeoutError::Timeout) => continue,
                                Err(flume::RecvTimeoutError::Disconnected) => break,
                            }
                        }
                    })
                    .expect("Failed to spawn driver thread");

                *driver_handle = Some(handle);
            }
        }
    }

    pub fn stop(&mut self) {
        match self {
            Runner::Scheduled { scheduler_handle, .. } => {
                if let Some(handle) = scheduler_handle.take() {
                    let _ = handle.join();
                }
            }
            Runner::Driven { driver_handle, .. } => {
                if let Some(handle) = driver_handle.take() {
                    let _ = handle.join();
                }
            }
        }
    }
}
