use std::{
    collections::VecDeque,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

use anyhow::Context as _;
use zeevonk::{
    Zeevonk,
    project::{ProjectFile as ZeevonkProjectFile, Stage},
};

use crate::pipeline::Pipeline;

mod cmd;
mod config;
mod event;
mod object;
mod pipeline;
mod selection;
mod snapshot;

pub use cmd::*;
pub use config::*;
pub use event::*;
pub use object::*;
pub use selection::*;
pub use snapshot::*;

pub use zeevonk as zv;

pub struct Engine {
    showfile_path: Option<PathBuf>,

    config: Config,
    objects: Objects,
    selection: Selection,

    command_queue: VecDeque<Command>,
    event_tx: crossbeam_channel::Sender<Event>,
    event_listener: EventListener,

    pipeline: Pipeline,
    zeevonk: Zeevonk,
}

impl Engine {
    pub fn new(showfile_path: Option<PathBuf>) -> anyhow::Result<Self> {
        let mut zv_project_file = ZeevonkProjectFile::default();
        let mut config = Config::default();
        let mut objects = Objects::default();

        match &showfile_path {
            Some(path) => {
                zv_project_file = ZeevonkProjectFile::load_from_folder(&path.join("zv/"))
                    .context("failed to load zeevonk project")?;

                config = serde_json::from_reader(
                    fs::File::open(path.join("config.json"))
                        .context("failed to open config file")?,
                )
                .context("failed to load config file")?;

                objects = serde_json::from_reader(
                    fs::File::open(path.join("objects.json"))
                        .context("failed to open objects file")?,
                )
                .context("failed to load objects file")?;
            }
            None => {}
        }

        let (event_tx, event_rx) = crossbeam_channel::unbounded();
        let event_listener = EventListener::new(event_rx);

        Ok(Self {
            showfile_path,

            pipeline: Pipeline::new(&config).context("failed to build pipeline")?,
            zeevonk: Zeevonk::new(zv_project_file)
                .context("failed to initialize zeevonk engine")?,

            selection: Selection::new(),
            config,
            objects,

            command_queue: VecDeque::new(),
            event_tx,
            event_listener,
        })
    }

    pub fn spawn(mut self) -> RunningEngine {
        let (tx, rx) = crossbeam_channel::unbounded::<EngineThreadCommand>();
        let (snapshot_tx, snapshot_rx) = crossbeam_channel::unbounded::<Arc<EngineSnapshot>>();

        let events = self.event_listener.clone();

        let join = thread::spawn(move || {
            self.run_threaded(rx, snapshot_tx);
        });

        let initial_snapshot =
            snapshot_rx.recv().expect("engine thread should send initial snapshot");

        let latest_snapshot = Arc::new(std::sync::RwLock::new(initial_snapshot));

        {
            let latest_snapshot = Arc::clone(&latest_snapshot);
            thread::spawn(move || {
                while let Ok(snapshot) = snapshot_rx.recv() {
                    if let Ok(mut guard) = latest_snapshot.write() {
                        *guard = snapshot;
                    }
                }
            });
        }

        let handle = EngineHandle { tx, events, latest_snapshot };

        RunningEngine::new(handle, join)
    }

    fn run_threaded(
        &mut self,
        rx: crossbeam_channel::Receiver<EngineThreadCommand>,
        snapshot_tx: crossbeam_channel::Sender<Arc<EngineSnapshot>>,
    ) {
        self.zeevonk.start();

        let period = Duration::from_secs_f64(1.0 / 60.0 as f64);
        let mut next_tick = Instant::now() + period;

        let config = Arc::new(self.config.clone());
        let stage = Arc::new(self.zeevonk.project().stage().clone());
        let mut objects = Arc::new(self.objects.clone());
        let mut selection = Arc::new(self.selection.clone());

        let _ = snapshot_tx.send(Arc::new(EngineSnapshot {
            showfile_path: self.showfile_path.clone(),
            stage: Arc::clone(&stage),
            config: Arc::clone(&config),
            objects: Arc::clone(&objects),
            selection: Arc::clone(&selection),
        }));

        loop {
            let now = Instant::now();

            if now < next_tick {
                match rx.recv_timeout(next_tick - now) {
                    Ok(EngineThreadCommand::Command(command)) => {
                        self.command_queue.push_back(command);

                        while let Ok(msg) = rx.try_recv() {
                            match msg {
                                EngineThreadCommand::Command(command) => {
                                    self.command_queue.push_back(command)
                                }
                                EngineThreadCommand::Shutdown => return,
                            }
                        }

                        let mut mutated_state = false;
                        while let Some(queued_command) = self.command_queue.pop_front() {
                            mutated_state = true;
                            if let Err(err) = self.execute(queued_command) {
                                log::error!("failed to execute queued command: {err}");
                            }
                        }

                        if mutated_state {
                            objects = Arc::new(self.objects.clone());
                            selection = Arc::new(self.selection.clone());

                            let _ = snapshot_tx.send(Arc::new(EngineSnapshot {
                                showfile_path: self.showfile_path.clone(),
                                stage: Arc::clone(&stage),
                                config: Arc::clone(&config),
                                objects: Arc::clone(&objects),
                                selection: Arc::clone(&selection),
                            }));
                        }

                        continue;
                    }
                    Ok(EngineThreadCommand::Shutdown) => break,
                    Err(crossbeam_channel::RecvTimeoutError::Timeout) => {}
                    Err(crossbeam_channel::RecvTimeoutError::Disconnected) => break,
                }
            }

            let mut mutated_state = false;

            let trigger_commands = match self.pipeline.resolve_triggers() {
                Ok(commands) => commands,
                Err(err) => {
                    log::error!("failed to resolve triggers: {err}");
                    let after = Instant::now();
                    while next_tick <= after {
                        next_tick += period;
                    }
                    continue;
                }
            };

            if !trigger_commands.is_empty() {
                mutated_state = true;
                for command in trigger_commands {
                    if let Err(err) = self.execute(command) {
                        log::error!("failed to execute command: {err}");
                    };
                }
            }

            while let Ok(msg) = rx.try_recv() {
                match msg {
                    EngineThreadCommand::Command(command) => self.command_queue.push_back(command),
                    EngineThreadCommand::Shutdown => return,
                }
            }

            while let Some(queued_command) = self.command_queue.pop_front() {
                mutated_state = true;
                if let Err(err) = self.execute(queued_command) {
                    log::error!("failed to execute queued command: {err}");
                }
            }

            let output = match self.pipeline.compose(&self.objects, self.zeevonk.project().stage())
            {
                Ok(output) => output,
                Err(err) => {
                    log::error!("failed to composite: {err}");
                    let after = Instant::now();
                    while next_tick <= after {
                        next_tick += period;
                    }
                    continue;
                }
            };

            self.zeevonk.clear_attribute_values();
            self.zeevonk.set_attribute_values(output);

            if mutated_state {
                objects = Arc::new(self.objects.clone());
                selection = Arc::new(self.selection.clone());

                let _ = snapshot_tx.send(Arc::new(EngineSnapshot {
                    showfile_path: self.showfile_path.clone(),
                    stage: Arc::clone(&stage),
                    config: Arc::clone(&config),
                    objects: Arc::clone(&objects),
                    selection: Arc::clone(&selection),
                }));
            }

            let after = Instant::now();
            while next_tick <= after {
                next_tick += period;
            }
        }
    }

    pub fn showfile_path(&self) -> Option<&Path> {
        self.showfile_path.as_deref()
    }

    pub fn stage(&self) -> &Stage {
        self.zeevonk.project().stage()
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn set_config(&mut self, config: Config) -> anyhow::Result<()> {
        self.pipeline = Pipeline::new(&config).context("failed to build pipeline")?;
        self.config = config;
        Ok(())
    }

    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    pub fn objects(&self) -> &Objects {
        &self.objects
    }

    fn execute(&mut self, command: Command) -> anyhow::Result<()> {
        command.execute(self)
    }

    pub fn event_listener(&self) -> &EventListener {
        &self.event_listener
    }

    pub(crate) fn emit_event(&self, event: Event) {
        let _ = self.event_tx.send(event);
    }
}

#[derive(Debug)]
enum EngineThreadCommand {
    Command(Command),
    Shutdown,
}

pub struct RunningEngine {
    handle: EngineHandle,
    join: Option<thread::JoinHandle<()>>,
}

impl RunningEngine {
    pub(crate) fn new(handle: EngineHandle, join: thread::JoinHandle<()>) -> Self {
        Self { handle, join: Some(join) }
    }

    pub fn handle(&self) -> &EngineHandle {
        &self.handle
    }

    pub fn join(mut self) -> thread::Result<()> {
        self.join.take().expect("join handle already taken").join()
    }
}

impl Drop for RunningEngine {
    fn drop(&mut self) {
        self.handle.shutdown();

        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

impl Clone for RunningEngine {
    fn clone(&self) -> Self {
        Self { handle: self.handle.clone(), join: None }
    }
}

impl std::fmt::Debug for RunningEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RunningEngine").finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
pub struct EngineHandle {
    pub(crate) tx: crossbeam_channel::Sender<EngineThreadCommand>,
    pub(crate) events: EventListener,
    pub(crate) latest_snapshot: Arc<RwLock<Arc<EngineSnapshot>>>,
}

impl EngineHandle {
    pub fn execute(&self, command: Command) {
        let _ = self.tx.send(EngineThreadCommand::Command(command));
    }

    pub fn shutdown(&self) {
        let _ = self.tx.send(EngineThreadCommand::Shutdown);
    }

    pub fn event_listener(&self) -> &EventListener {
        &self.events
    }

    pub fn snapshot(&self) -> Arc<EngineSnapshot> {
        self.latest_snapshot.read().expect("snapshot lock poisoned").clone()
    }
}
