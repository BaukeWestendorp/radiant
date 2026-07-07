use std::sync::{Arc, RwLock};
use std::time::Duration;

use anyhow::Context;
use libftd2xx::{BitsPerWord, Ftdi, FtdiCommon, Parity, StopBits, TimeoutError};

const BAUDRATE: u32 = 250000;
const BITS_8: BitsPerWord = BitsPerWord::Bits8;
const STOP_BITS_2: StopBits = StopBits::Bits2;
const PARITY_NONE: Parity = Parity::No;
const READ_TIMEOUT: Duration = Duration::from_millis(1000);
const WRITE_TIMEOUT: Duration = Duration::from_millis(1000);

use crate::dmx::{Multiverse, UniverseId};
use crate::output::EnttecDmxOutputInstanceDefinition;
use crate::service::ServiceDelegate;

pub struct EnttecInstanceService {
    universe_id: UniverseId,
    serial_number: String,

    multiverse: Arc<RwLock<Multiverse>>,

    ftdi: Option<RwLock<Ftdi>>,
}

impl EnttecInstanceService {
    pub fn new(
        definition: EnttecDmxOutputInstanceDefinition,
        multiverse: Arc<RwLock<Multiverse>>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            universe_id: definition.universe_id,
            serial_number: definition.serial_number,

            multiverse,

            ftdi: None,
        })
    }
}

impl ServiceDelegate for EnttecInstanceService {
    fn on_start(&self, _tick_tx: flume::Sender<()>) -> anyhow::Result<()> {
        let mut ftdi = Ftdi::with_serial_number(&self.serial_number).with_context(|| {
            format!("Failed to open FTDI device, possible devices: {:?}", libftd2xx::list_devices())
        })?;

        ftdi_init(&mut ftdi).context("Failed to initialize FTDI device")?;

        Ok(())
    }

    fn on_tick(&self) -> anyhow::Result<()> {
        let Some(ftdi) = self.ftdi.as_ref() else {
            log::error!("Enttec instance '{}' FTDI device not initialized", self.serial_number);
            return Ok(());
        };

        let mut ftdi =
            ftdi.write().map_err(|err| anyhow::anyhow!("Failed to acquire FTDI lock: {err}"))?;

        if let Err(err) = handle_frame(
            &mut ftdi,
            &self.universe_id,
            self.multiverse
                .read()
                .map_err(|err| anyhow::anyhow!("Failed to acquire multiverse lock: {err}"))?
                .clone(),
        ) {
            log::error!("Enttec instance '{}' failed to send frame: {err}", self.serial_number);
        }

        Ok(())
    }

    fn on_stop(&self) -> anyhow::Result<()> {
        let Some(ftdi) = self.ftdi.as_ref() else {
            log::error!("Enttec instance '{}' FTDI device not initialized", self.serial_number);
            return Ok(());
        };

        let mut ftdi =
            ftdi.write().map_err(|err| anyhow::anyhow!("Failed to acquire FTDI lock: {err}"))?;

        ftdi_close(&mut ftdi)
    }

    fn name(&self) -> &'static str {
        "EnttecOpenDmx"
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
