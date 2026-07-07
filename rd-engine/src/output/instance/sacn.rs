use std::sync::{Arc, RwLock};

use uuid::Uuid;

use crate::service::ServiceDelegate;
use crate::{
    dmx::{Multiverse, UniverseId},
    output::{
        SacnDmxOutputInstanceDefinition,
        protocol::sacn::{self, Universe},
    },
};

pub struct SacnInstanceService {
    definition: SacnDmxOutputInstanceDefinition,

    multiverse: Arc<RwLock<Multiverse>>,

    sacn_source: RwLock<Option<sacn::source::Source>>,
}

impl SacnInstanceService {
    pub fn new(
        definition: SacnDmxOutputInstanceDefinition,
        multiverse: Arc<RwLock<Multiverse>>,
    ) -> anyhow::Result<Self> {
        Ok(Self { definition, multiverse, sacn_source: RwLock::new(None) })
    }
}

impl ServiceDelegate for SacnInstanceService {
    fn on_start(&self) -> anyhow::Result<()> {
        let ip = self.definition.target_address.ip();
        let port = self.definition.target_address.port();

        let sacn_source = sacn::source::Source::new(sacn::source::SourceConfig {
            // FIXME: We should find a way to make this unique for each device, without it changing over time.
            cid: Uuid::new_v4(),
            name: self.definition.name.to_owned(),
            // FIXME: Implement multicasting.
            ip,
            port,
            priority: self.definition.priority,
            preview_data: self.definition.preview_mode,
            synchronization_address: 0,
            force_synchronization: false,
        })?;

        let mut lock = self
            .sacn_source
            .write()
            .map_err(|err| anyhow::anyhow!("Failed to acquire sACN source lock: {err}"))?;

        *lock = Some(sacn_source);

        Ok(())
    }

    fn on_tick(&self) -> anyhow::Result<()> {
        let mut lock = self
            .sacn_source
            .write()
            .map_err(|err| anyhow::anyhow!("Failed to acquire sACN source lock: {err}"))?;

        let Some(sacn_source) = lock.as_mut() else {
            log::error!("sACN instance not initialized");
            return Ok(());
        };

        if let Err(err) = handle_frame(
            sacn_source,
            &self.definition.universe_ids,
            self.multiverse
                .read()
                .map_err(|err| anyhow::anyhow!("Failed to acquire multiverse lock: {err}"))?
                .clone(),
        ) {
            log::error!("sACN instance failed to send frame: {err}");
        }

        Ok(())
    }

    fn on_stop(&self) -> anyhow::Result<()> {
        let mut lock = self
            .sacn_source
            .write()
            .map_err(|err| anyhow::anyhow!("Failed to acquire sACN source lock: {err}"))?;

        if let Some(sacn_source) = lock.take() {
            if let Err(err) = sacn_source.shutdown() {
                log::error!("sACN instance failed to shut down cleanly: {err}");
            }
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "sACN Instance"
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
