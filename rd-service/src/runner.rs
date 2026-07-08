use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

pub trait Runner: Send + 'static {
    fn start(
        &mut self,
        running: Arc<AtomicBool>,
        notify_tx: flume::Sender<()>,
    ) -> crate::Result<()>;
}

pub struct Scheduled {
    interval: Duration,
}

impl Scheduled {
    pub fn new(interval: Duration) -> Self {
        Self { interval }
    }

    pub fn interval(&self) -> Duration {
        self.interval
    }
}

impl Runner for Scheduled {
    fn start(
        &mut self,
        running: Arc<AtomicBool>,
        notify_tx: flume::Sender<()>,
    ) -> crate::Result<()> {
        let sleeper = spin_sleep::SpinSleeper::default();
        let mut next_tick = Instant::now() + self.interval;

        while running.load(Ordering::SeqCst) {
            let now = Instant::now();
            if now < next_tick {
                sleeper.sleep_until(next_tick);
            } else {
                let deviation = (now - next_tick).as_secs_f64();
                if now > next_tick + self.interval {
                    let ticks_missed = (deviation / self.interval.as_secs_f64()).floor() as u32 + 1;
                    next_tick += self.interval * ticks_missed;
                }
            }
            let _ = notify_tx.try_send(());
            next_tick += self.interval;
        }

        Ok(())
    }
}

pub struct Notified {
    notify_rx: flume::Receiver<()>,
}

impl Notified {
    pub fn new(notify_rx: flume::Receiver<()>) -> Self {
        Self { notify_rx }
    }
}

impl Runner for Notified {
    fn start(
        &mut self,
        running: Arc<AtomicBool>,
        notify_tx: flume::Sender<()>,
    ) -> crate::Result<()> {
        while running.load(Ordering::SeqCst) {
            match self.notify_rx.recv_timeout(Duration::from_millis(50)) {
                Ok(_) => {
                    let _ = notify_tx.try_send(());
                }
                Err(flume::RecvTimeoutError::Timeout) => continue,
                Err(flume::RecvTimeoutError::Disconnected) => {
                    log::warn!("Notify channel disconnected, stopping runner.");
                    break;
                }
            }
        }

        Ok(())
    }
}
