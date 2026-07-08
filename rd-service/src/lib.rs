use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};

mod error;
mod runner;

pub use error::*;
pub use runner::*;

pub struct Service<D: Delegate, R: Runner> {
    delegate: Arc<D>,
    runner: Option<R>,
    running: Arc<AtomicBool>,

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
        Self {
            delegate: Arc::new(delegate),
            runner: Some(runner),
            running: Arc::new(AtomicBool::new(false)),
            runner_handle: None,
            delegate_handle: None,
        }
    }

    pub fn delegate(&self) -> &D {
        self.delegate.as_ref()
    }

    pub fn start(&mut self) -> crate::Result<()> {
        if self.running.load(Ordering::SeqCst) {
            return Err(Error::ServiceAlreadyRunning);
        }

        let mut runner = self.runner.take().ok_or(Error::ServiceAlreadyRunning)?;

        self.running.store(true, Ordering::SeqCst);

        let (notify_tx, notify_rx) = flume::unbounded();

        self.runner_handle = Some(thread::spawn({
            let running = Arc::clone(&self.running);
            move || {
                if let Err(err) = runner.start(running, notify_tx) {
                    log::error!("Service runner failed: {err}");
                }
            }
        }));

        self.delegate_handle = Some(thread::spawn({
            let delegate = Arc::clone(&self.delegate);
            let running = Arc::clone(&self.running);
            move || {
                if let Err(err) = delegate.on_start() {
                    log::error!("Delegate start failed: {err}");
                    return;
                }

                while running.load(Ordering::SeqCst) {
                    match notify_rx.recv() {
                        Ok(notification) => {
                            if let Err(err) = delegate.on_frame(notification) {
                                log::error!("Delegate frame failed: {err}");
                            }
                        }
                        Err(_) => {
                            break;
                        }
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
        self.running.store(false, Ordering::SeqCst);

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
