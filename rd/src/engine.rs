use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use anyhow::Context;
use rd_service::{Scheduled, Service};

use crate::cmd::EngineRequest;
use crate::{
    Command, Commander, Event, Events, Project,
    services::{
        output::OutputService,
        trigger::{TriggerService, TriggerServiceRunner},
    },
};

/// Can be cloned safely as it acts like a handle.
#[derive(Clone)]
pub struct Engine {
    shared: Arc<EngineShared>,
}

struct EngineShared {
    inner: Arc<Mutex<EngineInner>>,
    commander: Commander,
    events: Events,
    request_tx: flume::Sender<EngineRequest>,
    request_thread: Mutex<Option<JoinHandle<()>>>,
}

impl Engine {
    pub fn new() -> Self {
        let (request_tx, request_rx) = flume::unbounded();
        let (event_tx, event_rx) = flume::unbounded();

        let commander = Commander::new(request_tx.clone());
        let events = Events::new(event_rx);

        let inner = Arc::new(Mutex::new(EngineInner {
            project: Default::default(),
            services: Services::default(),
            highlight: false,

            is_dirty: false,
            event_tx,
        }));

        let request_thread = thread::spawn({
            let inner = Arc::clone(&inner);
            let commander = commander.clone();
            move || {
                while let Ok(request) = request_rx.recv() {
                    match request {
                        EngineRequest::Execute { command, reply_tx } => {
                            let result = inner.lock().unwrap().execute(command);
                            log_request_error(&result, "Command execution failed");
                            if let Some(reply_tx) = reply_tx {
                                let _ = reply_tx.send(result);
                            }
                        }
                        EngineRequest::LoadProject { project, reply_tx } => {
                            let result = inner.lock().unwrap().load_project(project, &commander);
                            log_request_error(&result, "Project load failed");
                            let _ = reply_tx.send(result);
                        }
                        EngineRequest::ReplaceProject { project, reply_tx } => {
                            let result = inner.lock().unwrap().replace_project(project, &commander);
                            log_request_error(&result, "Project replace failed");
                            let _ = reply_tx.send(result);
                        }
                        EngineRequest::UnloadProject { reply_tx } => {
                            let result = inner.lock().unwrap().unload_project();
                            log_request_error(&result, "Project unload failed");
                            let _ = reply_tx.send(result);
                        }
                        EngineRequest::ReloadProject { reply_tx } => {
                            let result = inner.lock().unwrap().reload_project(&commander);
                            log_request_error(&result, "Project reload failed");
                            let _ = reply_tx.send(result);
                        }
                        EngineRequest::Stop => {
                            if let Err(err) = inner.lock().unwrap().stop() {
                                log::error!("Failed to stop engine: {err:#}");
                            }
                            break;
                        }
                    }
                }
            }
        });

        Self {
            shared: Arc::new(EngineShared {
                inner,
                commander,
                events,
                request_tx,
                request_thread: Mutex::new(Some(request_thread)),
            }),
        }
    }

    pub fn with_project<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Project) -> R,
    {
        let state = self.shared.inner.lock().unwrap();
        f(&state.project)
    }

    pub fn update_project<F, R>(&self, f: F) -> anyhow::Result<R>
    where
        F: FnOnce(&mut Project) -> R,
    {
        let mut project = self.with_project(Clone::clone);
        let result = f(&mut project);
        self.replace_project(project)?;

        Ok(result)
    }

    pub fn load_project(&self, project: Project) -> anyhow::Result<()> {
        self.request(|reply_tx| EngineRequest::LoadProject { project, reply_tx })
    }

    pub async fn load_project_async(&self, project: Project) -> anyhow::Result<()> {
        self.request_async(|reply_tx| EngineRequest::LoadProject { project, reply_tx }).await
    }

    pub fn replace_project(&self, project: Project) -> anyhow::Result<()> {
        self.request(|reply_tx| EngineRequest::ReplaceProject { project, reply_tx })
    }

    pub async fn replace_project_async(&self, project: Project) -> anyhow::Result<()> {
        self.request_async(|reply_tx| EngineRequest::ReplaceProject { project, reply_tx }).await
    }

    pub fn unload_project(&self) -> anyhow::Result<Project> {
        self.request(|reply_tx| EngineRequest::UnloadProject { reply_tx })
    }

    pub async fn unload_project_async(&self) -> anyhow::Result<Project> {
        self.request_async(|reply_tx| EngineRequest::UnloadProject { reply_tx }).await
    }

    pub fn reload_project(&self) -> anyhow::Result<()> {
        self.request(|reply_tx| EngineRequest::ReloadProject { reply_tx })
    }

    pub async fn reload_project_async(&self) -> anyhow::Result<()> {
        self.request_async(|reply_tx| EngineRequest::ReloadProject { reply_tx }).await
    }

    pub fn commander(&self) -> Commander {
        self.shared.commander.clone()
    }

    pub fn events(&self) -> Events {
        self.shared.events.clone()
    }

    pub fn execute(&self, command: Command) {
        self.shared.commander.execute(command);
    }

    pub fn execute_async(&self, command: Command) -> anyhow::Result<()> {
        self.request(|reply_tx| EngineRequest::Execute { command, reply_tx: Some(reply_tx) })
    }

    pub async fn execute_async_async(&self, command: Command) -> anyhow::Result<()> {
        self.request_async(|reply_tx| EngineRequest::Execute { command, reply_tx: Some(reply_tx) })
            .await
    }

    fn request<T>(
        &self,
        build_request: impl FnOnce(flume::Sender<anyhow::Result<T>>) -> EngineRequest,
    ) -> anyhow::Result<T> {
        let (reply_tx, reply_rx) = flume::bounded(1);
        self.shared
            .request_tx
            .send(build_request(reply_tx))
            .context("Engine request queue disconnected")?;
        reply_rx.recv().context("Engine request reply channel disconnected")?
    }

    async fn request_async<T>(
        &self,
        build_request: impl FnOnce(flume::Sender<anyhow::Result<T>>) -> EngineRequest,
    ) -> anyhow::Result<T> {
        let (reply_tx, reply_rx) = flume::bounded(1);
        self.shared
            .request_tx
            .send(build_request(reply_tx))
            .context("Engine request queue disconnected")?;
        reply_rx.recv_async().await.context("Engine request reply channel disconnected")?
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for EngineShared {
    fn drop(&mut self) {
        if self.request_tx.send(EngineRequest::Stop).is_err() {
            if let Err(err) = self.inner.lock().unwrap().stop() {
                log::error!("Failed to stop engine: {err:#}");
            }
        }

        if let Some(handle) = self.request_thread.lock().unwrap().take() {
            let _ = handle.join().map_err(|_| log::error!("Failed to join engine request thread"));
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

    pub fn replace_project(
        &mut self,
        project: Project,
        commander: &Commander,
    ) -> anyhow::Result<()> {
        if self.is_dirty {
            self.unload_project()?;
        }

        self.project = project;
        self.services = Services::new(&self.project, commander.clone());
        self.start()?;

        log::info!("Project replaced: '{}'", project_path_label(&self.project));

        self.emit(Event::ProjectLoaded);

        Ok::<(), anyhow::Error>(()).context("Could not replace project")
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

fn log_request_error<T>(result: &anyhow::Result<T>, context: &str) {
    if let Err(err) = result {
        log::error!("{context}: {err:#}");
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
