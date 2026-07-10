use std::ops::ControlFlow;

use rd_midi::MidiInputServiceRunner;
use rd_service::Service;

use crate::{
    Command, Commander, ExecutorButton, ExecutorId, project,
    services::trigger::midi::MidiTriggerService,
};

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
    midi_trigger_service: Option<Service<MidiTriggerService, MidiInputServiceRunner>>,

    trigger_rx: flume::Receiver<Trigger>,
}

impl TriggerServiceRunner {
    pub fn new(config: &project::TriggerConfig) -> Self {
        let (trigger_tx, trigger_rx) = flume::unbounded();

        let midi_trigger_service_runner = MidiInputServiceRunner::new();

        let midi_trigger_service = midi_trigger_service_runner
            .map(|runner| {
                Service::new(
                    MidiTriggerService::new(config.midi.clone(), trigger_tx.clone()),
                    runner,
                )
            })
            .map_err(|err| log::error!("Failed to initialize MIDI trigger service runner: {err:#}"))
            .ok();

        Self { midi_trigger_service, trigger_rx }
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
        if let Some(midi_trigger_service) = &mut self.midi_trigger_service {
            midi_trigger_service.start()?;
        }

        loop {
            match flume::Selector::new()
                .recv(&self.trigger_rx, |trigger| match trigger {
                    Ok(trigger) => {
                        let _ = notify_tx.send(trigger);
                        None
                    }
                    Err(flume::RecvError::Disconnected) => Some(ControlFlow::Break(())),
                })
                .recv(&stop_rx, |_| Some(ControlFlow::Break(())))
                .wait()
            {
                Some(ControlFlow::Break(())) => break,
                Some(ControlFlow::Continue(())) => continue,
                None => {}
            };
        }

        Ok(())
    }
}
