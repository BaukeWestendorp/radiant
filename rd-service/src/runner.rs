use std::{
    ops::ControlFlow,
    time::{Duration, Instant},
};

pub trait Runner {
    type Error: std::fmt::Display;
    type Data;

    fn start(
        &mut self,
        stop_rx: flume::Receiver<()>,
        notify_tx: flume::Sender<Self::Data>,
    ) -> Result<(), Self::Error>;
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
    type Error = crate::Error;
    type Data = Instant;

    fn start(
        &mut self,
        stop_rx: flume::Receiver<()>,
        notify_tx: flume::Sender<Self::Data>,
    ) -> Result<(), Self::Error> {
        let sleeper = spin_sleep::SpinSleeper::default();
        let mut next_tick = Instant::now() + self.interval;

        loop {
            match stop_rx.try_recv() {
                Ok(()) | Err(flume::TryRecvError::Disconnected) => break,
                Err(flume::TryRecvError::Empty) => {}
            }

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
            let _ = notify_tx.try_send(Instant::now());
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
    type Error = crate::Error;
    type Data = ();

    fn start(
        &mut self,
        stop_rx: flume::Receiver<()>,
        notify_tx: flume::Sender<Self::Data>,
    ) -> Result<(), Self::Error> {
        loop {
            match flume::Selector::new()
                .recv(&stop_rx, |_| Some(ControlFlow::Break(())))
                .recv(&self.notify_rx, |n| match n {
                    Ok(()) => {
                        let _ = notify_tx.send(());
                        None
                    }
                    Err(flume::RecvError::Disconnected) => {
                        log::warn!("Notify channel disconnected, stopping runner.");
                        Some(ControlFlow::Break(()))
                    }
                })
                .wait()
            {
                Some(ControlFlow::Break(())) => break,
                Some(ControlFlow::Continue(())) => continue,
                None => {}
            }
        }

        Ok(())
    }
}
