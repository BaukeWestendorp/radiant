use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use anyhow::Context;
use rd_service::{Scheduled, Service};

use crate::{
    Command, Commander, Project,
    services::{
        output::OutputService,
        trigger::{TriggerService, TriggerServiceRunner},
    },
};
use crate::{Event, Events};

pub struct Engine {
    inner: Arc<Mutex<EngineInner>>,
    commander: Commander,
    events: Events,

    // NOTE: We keep the handle so the thread isn't completely detached.
    _cmd_thread: JoinHandle<()>,
}

impl Engine {
    pub fn new() -> Self {
        let (cmd_tx, cmd_rx) = flume::unbounded();
        let (event_tx, event_rx) = flume::unbounded();

        let commander = Commander::new(cmd_tx);
        let events = Events::new(event_rx);

        let inner = Arc::new(Mutex::new(EngineInner {
            project: Default::default(),
            services: Services::default(),
            highlight: false,
            event_tx,
        }));

        let cmd_thread = thread::spawn({
            let inner = Arc::clone(&inner);
            move || {
                while let Ok(command) = cmd_rx.recv() {
                    let mut state = inner.lock().unwrap();
                    if let Err(e) = state.execute(command) {
                        log::error!("Command execution failed: {}", e);
                    }
                }
            }
        });

        Self { inner, commander, events, _cmd_thread: cmd_thread }
    }

    pub fn with_project<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Project) -> R,
    {
        let state = self.inner.lock().unwrap();
        f(&state.project)
    }

    pub fn load_project(&mut self, project: Project) -> anyhow::Result<()> {
        self.inner.lock().unwrap().load_project(project, &self.commander)
    }

    pub fn unload_project(&mut self) -> anyhow::Result<Project> {
        self.inner.lock().unwrap().unload_project()
    }

    pub fn reload_project(&mut self) -> anyhow::Result<()> {
        self.inner.lock().unwrap().reload_project(&self.commander)
    }

    pub fn commander(&self) -> Commander {
        self.commander.clone()
    }

    pub fn events(&self) -> Events {
        self.events.clone()
    }

    pub fn execute(&self, command: Command) -> anyhow::Result<()> {
        self.commander.execute(command);
        Ok(())
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        if let Err(err) = self.inner.lock().unwrap().stop() {
            log::error!("Failed to stop engine: {:?}", err);
        }
    }
}

struct EngineInner {
    project: Project,
    services: Services,
    highlight: bool,

    event_tx: flume::Sender<Event>,
}

impl EngineInner {
    pub fn load_project(&mut self, project: Project, commander: &Commander) -> anyhow::Result<()> {
        self.unload_project()?;
        self.project = project;
        self.services = Services::new(&self.project, commander.clone());
        self.start()?;
        Ok::<(), anyhow::Error>(()).context("Could not load project")
    }

    pub fn unload_project(&mut self) -> anyhow::Result<Project> {
        self.stop()?;
        self.services = Services::default();
        let old_project = std::mem::take(&mut self.project);
        Ok::<Project, anyhow::Error>(old_project).context("Could not unload project")
    }

    pub fn reload_project(&mut self, commander: &Commander) -> anyhow::Result<()> {
        let project = self.unload_project()?;
        self.load_project(project, commander)?;
        Ok::<(), anyhow::Error>(()).context("Could not reload project")
    }

    pub fn execute(&mut self, command: Command) -> anyhow::Result<()> {
        match &command {
            Command::HighlightToggle => {
                self.highlight = !self.highlight;
                self.emit(Event::HighlightChanged { highlight: self.highlight });
            }
            Command::Save { path } => {
                self.project
                    .save_to_folder()
                    .with_context(|| format!("Saving project to '{}'", path.display()))?;
                self.emit(Event::Saved { path: path.to_owned() });
            }
        }

        Ok::<(), anyhow::Error>(())
            .with_context(|| format!("Command '{:?}' could not be executed", command))
    }

    fn start(&mut self) -> anyhow::Result<()> {
        self.services.output.start().context("Output service failed to start")?;
        self.services.trigger.start().context("Trigger service failed to start")?;
        Ok::<(), anyhow::Error>(()).context("Services failed to start")
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        self.services.output.stop().context("Output service failed to stop")?;
        self.services.trigger.stop().context("Trigger service failed to stop")?;
        Ok::<(), anyhow::Error>(()).context("Services failed to stop")
    }

    fn emit(&mut self, event: Event) {
        let _ = self.event_tx.send(event);
    }
}

impl Drop for EngineInner {
    fn drop(&mut self) {
        if let Err(err) = self.stop() {
            log::error!("Failed to stop engine: {:?}", err);
        }
    }
}

struct Services {
    pub output: Service<OutputService, Scheduled>,
    pub trigger: Service<TriggerService, TriggerServiceRunner>,
}

impl Services {
    const DMX_OUTPUT_FREQUENCY: u32 = 40;
    const DMX_OUTPUT_INTERVAL: Duration =
        Duration::new(0, 1_000_000_000 / Self::DMX_OUTPUT_FREQUENCY);

    pub fn new(project: &Project, commander: Commander) -> Self {
        Self {
            output: Service::new(
                OutputService::new(&project.output),
                Scheduled::new(Self::DMX_OUTPUT_INTERVAL),
            ),
            trigger: Service::new(
                TriggerService::new(commander),
                TriggerServiceRunner::new(&project.trigger),
            ),
        }
    }
}

impl Default for Services {
    fn default() -> Self {
        Self {
            output: Service::new(
                OutputService::default(),
                Scheduled::new(Self::DMX_OUTPUT_INTERVAL),
            ),
            trigger: Service::new(
                TriggerService::new(Default::default()),
                TriggerServiceRunner::new(&Default::default()),
            ),
        }
    }
}
