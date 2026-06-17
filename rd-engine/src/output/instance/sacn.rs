use std::net::SocketAddr;
use std::sync::{
    Arc, RwLock,
    atomic::{AtomicBool, Ordering},
};
use std::thread::{self, JoinHandle};

use anyhow::Context as _;
use uuid::Uuid;

use crate::{
    dmx::{Multiverse, UniverseId},
    output::{
        SacnDmxOutputInstanceDefinition,
        protocol::sacn::{self, Universe},
    },
};

pub struct SacnInstance {
    name: String,
    universe_ids: Vec<UniverseId>,
    preview_mode: bool,
    priority: u8,
    target_address: SocketAddr,

    thread_handle: Option<JoinHandle<()>>,
    thread_running: Arc<AtomicBool>,
}

impl SacnInstance {
    pub fn new(definition: SacnDmxOutputInstanceDefinition) -> anyhow::Result<Self> {
        Ok(Self {
            name: definition.name,
            universe_ids: definition.universe_ids,
            preview_mode: definition.preview_mode,
            priority: definition.priority,
            target_address: definition.target_address,

            thread_handle: None,
            thread_running: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn start(
        &mut self,
        notify_rx: flume::Receiver<()>,
        multiverse: Arc<RwLock<Multiverse>>,
    ) -> anyhow::Result<()> {
        if self.thread_handle.is_some() {
            log::warn!("sACN instance '{}' thread already running", self.name);
            return Ok(());
        }

        let ip = self.target_address.ip();
        let port = self.target_address.port();

        let mut sacn_source = sacn::source::Source::new(sacn::source::SourceConfig {
            // FIXME: We should find a way to make this unique for each device, without it changing over time.
            cid: Uuid::new_v4(),
            name: self.name.to_owned(),
            // FIXME: Implement multicasting.
            ip,
            port,
            priority: self.priority,
            preview_data: self.preview_mode,
            synchronization_address: 0,
            force_synchronization: false,
        })?;

        let universe_ids = self.universe_ids.clone();
        let running = self.thread_running.clone();
        running.store(true, Ordering::SeqCst);

        let name = self.name.clone();
        let handle = thread::Builder::new()
            .name(format!("rd_sacn_{}", self.name))
            .spawn(move || {
                while running.load(Ordering::SeqCst) && notify_rx.recv().is_ok() {
                    let frame = multiverse.read().unwrap().clone();

                    if let Err(err) = handle_frame(&mut sacn_source, &universe_ids, frame) {
                        log::error!("sACN instance '{name}' failed to send frame: {err}");
                    }
                }

                if let Err(err) = sacn_source.shutdown() {
                    log::error!("sACN instance '{name}' failed to shut down cleanly: {err}");
                }
            })
            .context("Failed to spawn sACN instance thread")?;

        self.thread_handle = Some(handle);

        Ok(())
    }

    pub fn stop(&mut self) {
        if self.thread_handle.is_some() {
            self.thread_running.store(false, Ordering::SeqCst);
            if let Some(handle) = self.thread_handle.take() {
                let _ = handle.join();
            }
        }
    }
}

fn handle_frame(
    sacn_source: &mut sacn::source::Source,
    universe_ids: &[UniverseId],
    frame: Multiverse,
) -> anyhow::Result<()> {
    for universe_id in universe_ids {
        let Some(universe) = frame.universe(universe_id) else {
            continue;
        };

        let data = universe.values().map(|v| v.as_u8());

        let mut sacn_universe = Universe::new(universe_id.as_u16());
        sacn_universe.data_slots = data.into();

        // FIXME: Implement unicast.
        // FIXME: Implement synchronization.
        sacn_source.send_universe_data_packet(sacn_universe)?;
    }

    Ok(())
}
