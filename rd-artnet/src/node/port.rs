use std::{
    ops::ControlFlow,
    sync::Arc,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crate::{BoundNodeConfig, FrameScheduler, PortAddress, Universe};

pub enum WorkerHandle {
    Input { port_address: PortAddress, handle: JoinHandle<()> },
    Output { port_address: PortAddress, handle: JoinHandle<()> },
}

pub struct PortManager {
    pub workers: Vec<WorkerHandle>,
}

impl PortManager {
    pub fn new(inner: Arc<super::Inner>, stop_rx: flume::Receiver<()>) -> Self {
        let mut workers = Vec::new();

        for (bound_node, port) in inner.config.ports() {
            if let Some(port_address) = port.output() {
                let handle = Self::spawn_output_worker(
                    Arc::clone(&inner),
                    bound_node.clone(),
                    port_address,
                    port.physical(),
                    stop_rx.clone(),
                );
                workers.push(WorkerHandle::Output { port_address, handle });
            }

            if let Some(port_address) = port.input() {
                let handle = Self::spawn_input_worker(
                    Arc::clone(&inner),
                    bound_node.clone(),
                    port_address,
                    stop_rx.clone(),
                );
                workers.push(WorkerHandle::Input { port_address, handle });
            }
        }

        Self { workers }
    }

    fn spawn_output_worker(
        inner: Arc<super::Inner>,
        bound_node: BoundNodeConfig,
        port_address: PortAddress,
        physical: u8,
        stop_rx: flume::Receiver<()>,
    ) -> JoinHandle<()> {
        let frame_scheduler = bound_node.frame_scheduler().clone();
        let dmx_provider = bound_node.dmx_provider().clone();

        let send_output = move || {
            let mut universe = Universe::default();
            if let Err(err) = dmx_provider(&mut universe, port_address) {
                log::error!("DMX provider failed: {}", err);
            } else {
                // FIXME: Art-Net recommends a keep-alive of 800ms to 1000ms if
                // data is not changing, rather than continuously blasting at 40Hz,
                // unless the port's OutputStyle is specifically set to Continuous.
                if let Err(err) = inner.send_dmx(universe, port_address, physical) {
                    log::error!("Failed to send DMX data: {}", err);
                }
            }
        };

        thread::spawn(move || match frame_scheduler {
            FrameScheduler::Internal { refresh_rate } => {
                let interval = Duration::from_secs_f64(1.0 / refresh_rate as f64);
                let sleeper = spin_sleep::SpinSleeper::default();
                let mut next_tick = Instant::now() + interval;

                log::debug!("Started internal frame scheduler thread for port.");
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
                        if now > next_tick + interval {
                            let ticks_missed =
                                (deviation / interval.as_secs_f64()).floor() as u32 + 1;
                            next_tick += interval * ticks_missed;
                        }
                    }

                    send_output();

                    next_tick += interval;
                }
                log::debug!("Stopped internal frame scheduler thread");
            }
            FrameScheduler::External { notifier, .. } => {
                let (notify_tx, notify_rx) = flume::bounded(1);

                thread::spawn(move || {
                    log::debug!("Started external frame scheduler notifier thread for port.");
                    notifier(notify_tx);
                    log::debug!("Stopped external frame scheduler notifier thread");
                });

                loop {
                    match flume::Selector::new()
                        .recv(&stop_rx, |_| Some(ControlFlow::Break(())))
                        .recv(&notify_rx, |v| match v {
                            Ok(()) => {
                                send_output();
                                None
                            }
                            Err(flume::RecvError::Disconnected) => {
                                log::error!("Frame scheduler notifier disconnected");
                                return Some(ControlFlow::Break(()));
                            }
                        })
                        .wait()
                    {
                        Some(ControlFlow::Break(())) => break,
                        Some(ControlFlow::Continue(())) => continue,
                        None => {}
                    }
                }
            }
        })
    }

    fn spawn_input_worker(
        _inner: Arc<super::Inner>,
        _bound_node: BoundNodeConfig,
        _port_address: PortAddress,
        stop_rx: flume::Receiver<()>,
    ) -> JoinHandle<()> {
        // FIXME: This needs a channel connected to the main receiver thread.
        thread::spawn(move || {
            log::debug!("Started input worker thread for port.");
            loop {
                match stop_rx.try_recv() {
                    Ok(()) | Err(flume::TryRecvError::Disconnected) => break,
                    Err(flume::TryRecvError::Empty) => {}
                }

                // FIXME: Implement HTP / LTP data merging.
                // If ArtDmx arrives from two different IP addresses for the same
                // port, the data must be merged.

                // FIXME: Implement DMX input timeout. If no ArtDmx is received for this port
                // within a specific timeframe, `data_received` should be set back to `false`.

                thread::sleep(Duration::from_millis(100));
            }
            log::debug!("Stopped input worker thread");
        })
    }
}
