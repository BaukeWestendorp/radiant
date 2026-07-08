use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::Duration,
};

use rd_service::{Delegate, Scheduled, Service};

struct ExampleSchedulerDelegate {
    frames_processed: AtomicUsize,
}

impl Delegate for ExampleSchedulerDelegate {
    type Error = anyhow::Error;

    fn on_start(&self) -> Result<(), Self::Error> {
        println!("Scheduler started.");
        self.frames_processed.store(0, Ordering::SeqCst);
        Ok(())
    }

    fn on_frame(&self) -> Result<(), Self::Error> {
        self.frames_processed.fetch_add(1, Ordering::SeqCst);
        println!("Processed frame: {}", self.frames_processed.load(Ordering::SeqCst));
        Ok(())
    }

    fn on_stop(&self) -> Result<(), Self::Error> {
        println!(
            "Scheduler stopped after {} frames.",
            self.frames_processed.load(Ordering::SeqCst)
        );
        Ok(())
    }
}

fn main() -> rd_service::Result<()> {
    let mut service = Service::new(
        ExampleSchedulerDelegate { frames_processed: AtomicUsize::new(9) },
        Scheduled::new(Duration::from_secs_f64(1.0 / 44.0)),
    );

    println!("Starting service...");
    service.start()?;

    thread::sleep(Duration::from_millis(1000));

    println!("Stopping service...");
    service.stop()?;

    Ok(())
}
