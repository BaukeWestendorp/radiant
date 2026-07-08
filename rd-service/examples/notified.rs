use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::Duration,
};

use rd_service::{Delegate, Notified, Service};

struct EventTriggerDelegate {
    event_count: AtomicUsize,
}

impl Delegate for EventTriggerDelegate {
    type Error = anyhow::Error;

    fn on_start(&self) -> Result<(), Self::Error> {
        println!("Notified service has initialized.");
        self.event_count.store(0, Ordering::SeqCst);
        Ok(())
    }

    fn on_frame(&self) -> Result<(), Self::Error> {
        self.event_count.fetch_add(1, Ordering::SeqCst);
        println!(
            "Trigger event processed! Total events: {}",
            self.event_count.load(Ordering::SeqCst)
        );
        Ok(())
    }

    fn on_stop(&self) -> Result<(), Self::Error> {
        println!(
            "Notified service shut down. Processed {} events.",
            self.event_count.load(Ordering::SeqCst)
        );
        Ok(())
    }
}

fn main() -> rd_service::Result<()> {
    let (trigger_tx, trigger_rx) = flume::bounded(1);

    let mut service = Service::new(
        EventTriggerDelegate { event_count: AtomicUsize::new(0) },
        Notified::new(trigger_rx),
    );

    println!("Starting service...");
    service.start()?;

    for i in 1..=3 {
        thread::sleep(Duration::from_millis(150));
        println!("Firing external notification {}...", i);
        let _ = trigger_tx.send(());
    }

    thread::sleep(Duration::from_millis(100));

    println!("Stopping service...");
    service.stop()?;

    Ok(())
}
