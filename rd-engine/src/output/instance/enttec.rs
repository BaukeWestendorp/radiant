use std::sync::{
    Arc, RwLock,
    atomic::{AtomicBool, Ordering},
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use anyhow::Context as _;
use libftd2xx::{BitsPerWord, Ftdi, FtdiCommon, Parity, StopBits, TimeoutError};
use thread_priority::ThreadBuilderExt;

const BAUDRATE: u32 = 250000;
const BITS_8: BitsPerWord = BitsPerWord::Bits8;
const STOP_BITS_2: StopBits = StopBits::Bits2;
const PARITY_NONE: Parity = Parity::No;
const READ_TIMEOUT: Duration = Duration::from_millis(1000);
const WRITE_TIMEOUT: Duration = Duration::from_millis(1000);

const TARGET_OUTPUT_INTERVAL: Duration = Duration::from_millis(25);

use crate::{
    dmx::{Multiverse, UniverseId},
    output::EnttecDmxOutputInstanceDefinition,
};

pub struct EnttecInstance {
    universe_id: UniverseId,
    serial_number: String,

    thread_handle: Option<JoinHandle<()>>,
    thread_running: Arc<AtomicBool>,
}

impl EnttecInstance {
    pub fn new(definition: EnttecDmxOutputInstanceDefinition) -> anyhow::Result<Self> {
        Ok(Self {
            universe_id: definition.universe_id,
            serial_number: definition.serial_number,

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
            log::warn!("Enttec instance '{}' thread already running", self.serial_number);
            return Ok(());
        }

        let mut ftdi = Ftdi::with_serial_number(&self.serial_number).with_context(|| {
            format!("Failed to open FTDI device, possible devices: {:?}", libftd2xx::list_devices())
        })?;

        ftdi_init(&mut ftdi).context("Failed to initialize FTDI device")?;

        let universe_id = self.universe_id.clone();
        let running = self.thread_running.clone();
        running.store(true, Ordering::SeqCst);

        let serial = self.serial_number.clone();
        let handle = thread::Builder::new()
            .name(format!("enttec_open_dmx_{}", self.serial_number))
            .spawn_with_priority(thread_priority::ThreadPriority::Max, move |prio_result| {
                if prio_result.is_err() {
                    log::warn!(
                        "could not set {} thread priority to max",
                        thread::current().name().unwrap_or("<unnamed>")
                    );
                }

                while running.load(Ordering::SeqCst) && notify_rx.recv().is_ok() {
                    let frame = multiverse.read().unwrap().clone();
                    let start_time = std::time::Instant::now();

                    if let Err(err) = handle_frame(&mut ftdi, &universe_id, frame) {
                        log::error!("Enttec instance '{serial}' failed to send frame: {err}");
                    }

                    let elapsed = start_time.elapsed();
                    if elapsed < TARGET_OUTPUT_INTERVAL {
                        thread::sleep(TARGET_OUTPUT_INTERVAL - elapsed);
                    }
                }

                if let Err(err) = ftdi_close(&mut ftdi) {
                    log::error!("Enttec instance '{serial}' failed to shut down cleanly: {err}");
                }
            })
            .context("Failed to spawn Enttec instance thread")?;

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

    pub fn universe_id(&self) -> UniverseId {
        self.universe_id
    }

    pub fn serial_number(&self) -> &str {
        &self.serial_number
    }
}

fn handle_frame(
    ftdi: &mut Ftdi,
    universe_id: &UniverseId,
    frame: Multiverse,
) -> anyhow::Result<()> {
    let Some(universe) = frame.universe(universe_id) else {
        return Ok(());
    };

    let data: Vec<u8> = universe.values().map(|v| v.as_u8()).to_vec();

    ftdi_send(ftdi, &data)?;

    Ok(())
}

fn ftdi_init(ftdi: &mut Ftdi) -> anyhow::Result<()> {
    ftdi.reset()?;
    ftdi.set_baud_rate(BAUDRATE)?;
    ftdi.set_data_characteristics(BITS_8, STOP_BITS_2, PARITY_NONE)?;
    ftdi.set_timeouts(READ_TIMEOUT, WRITE_TIMEOUT)?;
    ftdi.set_flow_control_none()?;
    ftdi.clear_rts()?;
    ftdi.purge_rx()?;
    ftdi.purge_tx()?;
    Ok(())
}

fn ftdi_send(ftdi: &mut Ftdi, buffer: &[u8]) -> anyhow::Result<()> {
    ftdi.set_break_on()?;
    ftdi.set_break_off()?;
    ftdi.write(&[0])?; // We need to add this prefix byte to convert the buffer's 0-index to a 1-index.
    ftdi.write_all(buffer).map_err(|err| match err {
        TimeoutError::FtStatus(ft_status) => anyhow::anyhow!("FTDI write error: {:?}", ft_status),
        TimeoutError::Timeout { .. } => anyhow::anyhow!("FTDI write timeout"),
    })?;
    Ok(())
}

fn ftdi_close(ftdi: &mut Ftdi) -> anyhow::Result<()> {
    ftdi.close()?;
    Ok(())
}
