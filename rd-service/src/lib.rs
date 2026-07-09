use std::ops::ControlFlow;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

mod error;
mod runner;

pub use error::*;
pub use runner::*;

pub struct Service<D: Delegate, R: Runner> {
    delegate: Arc<D>,
    runner: Option<R>,

    stop_tx: flume::Sender<()>,
    stop_rx: flume::Receiver<()>,

    runner_handle: Option<JoinHandle<()>>,
    delegate_handle: Option<JoinHandle<()>>,
}

impl<D, R> Service<D, R>
where
    D: Delegate + Send + Sync + 'static,
    R: Runner<Data = D::Data> + Send + 'static,
    D::Data: Send + 'static,
{
    pub fn new(delegate: D, runner: R) -> Self {
        let (stop_tx, stop_rx) = flume::bounded(1);

        Self {
            delegate: Arc::new(delegate),
            runner: Some(runner),
            stop_tx,
            stop_rx,

            runner_handle: None,
            delegate_handle: None,
        }
    }

    pub fn delegate(&self) -> &D {
        self.delegate.as_ref()
    }

    pub fn start(&mut self) -> crate::Result<()> {
        let mut runner = self.runner.take().ok_or(Error::ServiceAlreadyRunning)?;

        let (notify_tx, notify_rx) = flume::unbounded();

        self.runner_handle = Some(thread::spawn({
            let stop_rx = self.stop_rx.clone();
            move || {
                if let Err(err) = runner.start(stop_rx, notify_tx) {
                    log::error!("Service runner failed: {err}");
                }
            }
        }));

        self.delegate_handle = Some(thread::spawn({
            let delegate = Arc::clone(&self.delegate);
            let stop_rx = self.stop_rx.clone();
            move || {
                if let Err(err) = delegate.on_start() {
                    log::error!("Delegate start failed: {err}");
                    return;
                }

                loop {
                    match flume::Selector::new()
                        .recv(&stop_rx, |_| Some(ControlFlow::Break(())))
                        .recv(&notify_rx, |n| match n {
                            Ok(notification) => {
                                if let Err(err) = delegate.on_frame(notification) {
                                    log::error!("Delegate frame failed: {err}");
                                }

                                None
                            }
                            Err(_) => Some(ControlFlow::Break(())),
                        })
                        .wait()
                    {
                        Some(ControlFlow::Continue(())) => continue,
                        Some(ControlFlow::Break(())) => break,
                        None => {}
                    }
                }

                if let Err(err) = delegate.on_stop() {
                    log::error!("Delegate stop failed: {err}");
                }
            }
        }));

        Ok(())
    }

    pub fn stop(&mut self) -> crate::Result<()> {
        let _ = self.stop_tx.send(());

        if let Some(handle) = self.runner_handle.take() {
            let _ = handle.join();
        }

        if let Some(handle) = self.delegate_handle.take() {
            let _ = handle.join();
        }

        Ok(())
    }
}

pub trait Delegate {
    type Error: std::fmt::Display;
    type Data;

    fn on_start(&self) -> std::result::Result<(), Self::Error>;
    fn on_frame(&self, data: Self::Data) -> std::result::Result<(), Self::Error>;
    fn on_stop(&self) -> std::result::Result<(), Self::Error>;
}
