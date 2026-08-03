use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

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

/// Can be cloned safely as it acts like a handle.
pub struct Engine {
    inner: Arc<Mutex<EngineInner>>,
    commander: Commander,
    events: Events,

    // NOTE: We keep the handle so the thread isn't completely detached.
    _cmd_thread: Arc<JoinHandle<()>>,
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

            is_dirty: false,
            event_tx,
        }));

        let cmd_thread = thread::spawn({
            let inner = Arc::clone(&inner);
            move || {
                while let Ok(command) = cmd_rx.recv() {
                    let mut state = inner.lock().unwrap();
                    if let Err(err) = state.execute(command) {
                        log::error!("Command execution failed: {err:#}");
                    }
                }
            }
        });

        Self { inner, commander, events, _cmd_thread: Arc::new(cmd_thread) }
    }

    pub fn with_project<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Project) -> R,
    {
        let state = self.inner.lock().unwrap();
        f(&state.project)
    }

    pub fn update_project<F, R>(&mut self, f: F) -> anyhow::Result<R>
    where
        F: FnOnce(&mut Project) -> R,
    {
        let mut project = self.unload_project()?;
        let result = (f)(&mut project);
        self.load_project(project)?;

        Ok(result)
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

    pub fn execute(&self, command: Command) {
        self.commander.execute(command);
    }
}

impl Clone for Engine {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            commander: Commander::clone(&self.commander),
            events: Events::clone(&self.events),
            _cmd_thread: Arc::clone(&self._cmd_thread),
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        if let Err(err) = self.inner.lock().unwrap().stop() {
            log::error!("Failed to stop engine: {err:#}");
        }
    }
}

struct EngineInner {
    project: Project,
    services: Services,
    highlight: bool,

    is_dirty: bool,
    event_tx: flume::Sender<Event>,
}

impl EngineInner {
    pub fn load_project(&mut self, project: Project, commander: &Commander) -> anyhow::Result<()> {
        let started_at = Instant::now();
        let project_path = project_path_label(&project);

        if self.is_dirty {
            self.unload_project()?;
        }
        self.project = project;
        self.services = Services::new(&self.project, commander.clone());
        self.start()?;

        log::info!("Project loaded in {:?}: '{}'", started_at.elapsed(), project_path);

        self.emit(Event::ProjectLoaded);

        Ok::<(), anyhow::Error>(()).context("Could not load project")
    }

    pub fn unload_project(&mut self) -> anyhow::Result<Project> {
        let started_at = Instant::now();
        let project_path = project_path_label(&self.project);

        self.stop()?;
        self.services = Services::default();
        let old_project = std::mem::take(&mut self.project);

        log::info!("Project unloaded in {:?}: '{}'", started_at.elapsed(), project_path);

        self.emit(Event::ProjectUnloaded);

        Ok::<Project, anyhow::Error>(old_project).context("Could not unload project")
    }

    pub fn reload_project(&mut self, commander: &Commander) -> anyhow::Result<()> {
        let started_at = Instant::now();
        let project_path = project_path_label(&self.project);

        let project = self.unload_project()?;
        self.load_project(project, commander)?;

        log::info!("Project reloaded in {:?}: '{}'", started_at.elapsed(), project_path);

        self.emit(Event::ProjectReloaded);

        Ok::<(), anyhow::Error>(()).context("Could not reload project")
    }

    pub fn execute(&mut self, command: Command) -> anyhow::Result<()> {
        self.is_dirty = true;
        match &command {
            Command::HighlightToggle => {
                self.highlight = !self.highlight;
                self.emit(Event::HighlightChanged { highlight: self.highlight });
            }
            Command::Save { path } => {
                let started_at = Instant::now();
                self.project
                    .save_to_folder()
                    .with_context(|| format!("Saving project to '{}'", path.display()))?;
                log::info!("Project saved in {:?}: '{}'", started_at.elapsed(), path.display());
                self.emit(Event::Saved { path: path.to_owned() });
                self.is_dirty = false;
            }
        }

        log::info!("Command executed: {:?}", command);

        Ok::<(), anyhow::Error>(())
            .with_context(|| format!("Command '{:?}' could not be executed", command))
    }

    fn start(&mut self) -> anyhow::Result<()> {
        let started_at = Instant::now();

        let output_started_at = Instant::now();
        self.services.output.start().context("Output service failed to start")?;
        log::info!("Output service started in {:?}", output_started_at.elapsed());

        let trigger_started_at = Instant::now();
        self.services.trigger.start().context("Trigger service failed to start")?;
        log::info!("Trigger service started in {:?}", trigger_started_at.elapsed());

        log::info!("All services started in {:?}", started_at.elapsed());
        Ok::<(), anyhow::Error>(()).context("Services failed to start")
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        let started_at = Instant::now();

        let output_started_at = Instant::now();
        self.services.output.stop().context("Output service failed to stop")?;
        log::info!("Output service stopped in {:?}", output_started_at.elapsed());

        let trigger_started_at = Instant::now();
        self.services.trigger.stop().context("Trigger service failed to stop")?;
        log::info!("Trigger service stopped in {:?}", trigger_started_at.elapsed());

        log::info!("All services stopped in {:?}", started_at.elapsed());
        Ok::<(), anyhow::Error>(()).context("Services failed to stop")
    }

    fn emit(&mut self, event: Event) {
        log::debug!("Event emitted: {:?}", event);
        let _ = self.event_tx.send(event);
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

fn project_path_label(project: &Project) -> String {
    project
        .path
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "<unsaved project>".to_string())
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
