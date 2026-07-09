use anyhow::Context;

use crate::{Command, Commander, ExecutorButton, ExecutorId, project};

mod midi;

#[derive(Debug, Clone)]
pub enum Trigger {
    ToggleHighlight,

    ExecutorMaster { executor_id: ExecutorId, value: f32 },
    ExecutorButton { executor_id: ExecutorId, button: ExecutorButton, pressed: bool },

    EncoderSetValue { encoder_ix: usize, value: f32 },
}

pub struct TriggerService {
    commander: Commander,
}

impl TriggerService {
    pub fn new(commander: Commander) -> Self {
        Self { commander }
    }
}

impl Default for TriggerService {
    fn default() -> Self {
        Self { commander: Commander::default() }
    }
}

impl rd_service::Delegate for TriggerService {
    type Error = anyhow::Error;
    type Data = Trigger;

    fn on_start(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_frame(&self, trigger: Trigger) -> Result<(), Self::Error> {
        match trigger {
            Trigger::ToggleHighlight => {
                self.commander.execute(Command::HighlightToggle);
            }

            Trigger::ExecutorMaster { executor_id, value } => {
                log::error!("FIXME: Implement handling ExecutorMaster {} {}", executor_id, value);
            }
            Trigger::ExecutorButton { executor_id, button, pressed } => {
                log::error!(
                    "FIXME: Implement handling ExecutorButton {} {:?} {}",
                    executor_id,
                    button,
                    pressed
                );
            }

            Trigger::EncoderSetValue { encoder_ix, value } => {
                log::error!("FIXME: Implement handling EncoderSetValue {} {}", encoder_ix, value);
            }
        };

        Ok(())
    }

    fn on_stop(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub struct TriggerServiceRunner {
    config: project::TriggerConfig,
}

impl TriggerServiceRunner {
    pub fn new(config: &project::TriggerConfig) -> Self {
        Self { config: config.clone() }
    }
}

impl rd_service::Runner for TriggerServiceRunner {
    type Error = anyhow::Error;
    type Data = Trigger;

    fn start(
        &mut self,
        stop_rx: flume::Receiver<()>,
        notify_tx: flume::Sender<Self::Data>,
    ) -> Result<(), Self::Error> {
        midi::start_listener(stop_rx, notify_tx, &self.config.midi)
            .context("Failed to start MIDI listener")?;

        Ok(())
    }
}
