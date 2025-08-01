use std::cell::RefCell;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use eyre::Context;
use spin_sleep::SpinSleeper;

use crate::error::Result;
use crate::pipeline::Pipeline;
use crate::protocols::sacn;

// FIXME: We should find a way to create a unique UUID for a device, without it
// changing over it's lifetime.
const SACN_CID: sacn::ComponentIdentifier = sacn::ComponentIdentifier::from_bytes([
    0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
]);

const SYNCHRONIZATION_ADDRESS: u16 = 0;
const FORCE_SYNCHRONIZATION: bool = false;

const DMX_OUTPUT_FRAME_TIME: Duration = Duration::from_millis(40);

pub(crate) struct Protocols {
    pipeline: Arc<Mutex<Pipeline>>,
    tx: crossbeam_channel::Sender<()>,
    rx: crossbeam_channel::Receiver<()>,
    sacn_sources: RefCell<Vec<JoinHandle<()>>>,
}

impl Protocols {
    pub fn new(pipeline: Arc<Mutex<Pipeline>>) -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        Self { pipeline, tx, rx, sacn_sources: RefCell::new(Vec::new()) }
    }

    pub fn start(&self) {
        let tx = self.tx.clone();
        thread::Builder::new()
            .name("protocol_handler".to_string())
            .spawn(move || {
                let sleeper = SpinSleeper::default();

                loop {
                    let start = Instant::now();

                    tx.send(()).expect("should new frame notifier to protocols");

                    let elapsed = start.elapsed();
                    if elapsed < DMX_OUTPUT_FRAME_TIME {
                        sleeper.sleep(DMX_OUTPUT_FRAME_TIME - elapsed);
                    }
                }
            })
            .unwrap();
    }

    pub fn add_sacn_source(
        &self,
        name: String,
        ip: IpAddr,
        priority: u8,
        preview_data: bool,
    ) -> Result<()> {
        let source = sacn::Source::new(sacn::SourceConfig {
            cid: SACN_CID,
            name,
            ip,
            port: sacn::DEFAULT_PORT,
            priority,
            preview_data,
            synchronization_address: SYNCHRONIZATION_ADDRESS,
            force_synchronization: FORCE_SYNCHRONIZATION,
        })
        .wrap_err("could not create sACN source")?;

        self.spawn_sacn_source_thread(source);

        Ok(())
    }

    fn spawn_sacn_source_thread(&self, source: sacn::Source) {
        let rx = self.rx.clone();
        let pipeline = self.pipeline.clone();
        let handle = thread::spawn(move || {
            while let Ok(()) = rx.recv() {
                let multiverse = pipeline.lock().unwrap().resolved_multiverse().clone();
                for (id, universe) in multiverse.universes() {
                    let mut sacn_universe = sacn::Universe::new(**id);
                    sacn_universe.data_slots = universe.values().iter().map(|v| v.0).collect();
                    source
                        .send_universe_data_packet(sacn_universe)
                        .map_err(|err| log::error!("failed to send universe data over sACN: {err}"))
                        .ok();
                }
            }
        });

        self.sacn_sources.borrow_mut().push(handle);
    }
}
