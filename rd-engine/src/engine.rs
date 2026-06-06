use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use arc_swap::ArcSwap;
use crossbeam_channel::{Receiver, Sender};

use crate::{
    Project,
    cmd::Command,
    event::{Event, EventListener},
    object::Objects,
    output::{Output, OutputDefinition},
    patch::Patch,
    pipeline::Pipeline,
    selection::Selection,
    trigger::{Triggers, TriggersDefinition},
};

pub struct Engine {
    showfile_path: Option<PathBuf>,

    pub(crate) patch: Patch,
    pub(crate) output: Output,
    pub(crate) triggers: Triggers,
    pub(crate) objects: Objects,
    pub(crate) selection: Selection,
    pub(crate) highlight: bool,

    snapshot: EngineSnapshot,

    event_tx: crossbeam_channel::Sender<Event>,
    event_listener: EventListener,

    pipeline: Pipeline,
}

impl Engine {
    pub fn new(project: Project) -> anyhow::Result<Self> {
        let patch = Patch::new(
            project.patch().clone(),
            project.file().map(|f| f.gdtfs()).unwrap_or(&HashMap::new()),
        )?;
        let output = Output::new(project.output().clone())?;
        let triggers = Triggers::new(project.triggers().clone())?;
        let objects = project.objects().clone();

        let snapshot = EngineSnapshot::default();

        let (event_tx, event_rx) = crossbeam_channel::unbounded();
        let event_listener = EventListener::new(event_rx);

        let pipeline = Pipeline::new(&patch);

        let mut engine = Self {
            showfile_path: project.file().map(|p| p.path().clone()),

            patch,
            output,
            triggers,
            snapshot,
            objects,

            event_tx,
            event_listener,

            selection: Selection::new(),
            highlight: false,

            pipeline,
        };

        engine.update_snapshot();

        Ok(engine)
    }

    pub fn showfile_path(&self) -> Option<&Path> {
        self.showfile_path.as_deref()
    }

    pub fn patch(&self) -> &Patch {
        &self.patch
    }

    pub fn output(&self) -> &Output {
        &self.output
    }

    pub fn objects(&self) -> &Objects {
        &self.objects
    }

    pub fn event_listener(&self) -> EventListener {
        self.event_listener.clone()
    }

    pub fn execute(&mut self, command: Command) -> anyhow::Result<()> {
        let res = command.execute(self);
        self.update_snapshot();
        res
    }

    pub fn snapshot(&self) -> EngineSnapshot {
        self.snapshot.clone()
    }

    fn update_snapshot(&mut self) {
        self.snapshot = EngineSnapshot {
            showfile_path: self.showfile_path.clone(),
            patch: Arc::new(self.patch.clone()),
            output: Arc::new(self.output.definition().clone()),
            triggers: Arc::new(self.triggers.definition().clone()),
            objects: Arc::new(self.objects.clone()),
            selection: Arc::new(self.selection.clone()),
            highlight: self.highlight,
        };
    }

    pub(crate) fn emit(&self, event: Event) {
        let _ = self.event_tx.send(event);
    }

    fn run(mut self, rx: Receiver<EngineMessage>, snapshot_store: Arc<ArcSwap<EngineSnapshot>>) {
        const INTERVAL: Duration = Duration::new(0, ((1_000_000_000_f64 / 60.0).round()) as u32);

        self.output.start();

        let mut next_tick = Instant::now() + INTERVAL;
        let mut running = true;

        while running {
            while let Ok(msg) = rx.try_recv() {
                running = self.handle_message(msg, &snapshot_store);
                if !running {
                    break;
                }
            }

            if !running {
                break;
            }

            let now = Instant::now();
            if now < next_tick {
                crossbeam_channel::select! {
                    recv(rx) -> msg => {
                        if let Ok(msg) = msg {
                            running = self.handle_message(msg, &snapshot_store);
                        } else {
                            running = false;
                        }
                    }
                    recv(crossbeam_channel::after(next_tick - now)) -> _ => {}
                }
                continue;
            }

            let now = Instant::now();
            if now > next_tick + INTERVAL {
                let deviation = (now - next_tick).as_secs_f64();
                let ticks_missed = (deviation / INTERVAL.as_secs_f64()).floor() as u32 + 1;
                next_tick += INTERVAL * ticks_missed;
            }

            self.tick(&snapshot_store);
            next_tick += INTERVAL;
        }

        self.output.stop();
    }

    fn handle_message(
        &mut self,
        msg: EngineMessage,
        snapshot_store: &Arc<ArcSwap<EngineSnapshot>>,
    ) -> bool {
        match msg {
            EngineMessage::Command { command, resp } => {
                let res = self.execute(command);
                snapshot_store.store(Arc::new(self.snapshot()));
                if let Some(resp) = resp {
                    let _ = resp.send(res);
                }
                true
            }
            EngineMessage::Shutdown { resp } => {
                if let Some(resp) = resp {
                    let _ = resp.send(());
                }
                false
            }
        }
    }

    fn tick(&mut self, snapshot_store: &Arc<ArcSwap<EngineSnapshot>>) {
        let commands = self.pipeline.resolve_triggers(&self.triggers);
        let mut snapshot_dirty = !commands.is_empty();
        for command in commands {
            if let Err(err) = self.execute(command) {
                snapshot_dirty = true;
                log::error!("Failed to execute command: {err}");
            }
        }
        if snapshot_dirty {
            snapshot_store.store(Arc::new(self.snapshot()));
        }

        let attributes = match self.pipeline.resolve_attributes(&self.objects, &self.patch) {
            Ok(attributes) => attributes,
            Err(err) => {
                log::error!("Failed to resolve attribute values: {err}");
                return;
            }
        };

        let highlighted_fixtures = self.highlight.then(|| self.selection.fixture_ids());

        let multiverse = self.pipeline.resolve_dmx(&attributes, highlighted_fixtures);

        self.output.update(multiverse);
    }
}

enum EngineMessage {
    Command { command: Command, resp: Option<Sender<anyhow::Result<()>>> },
    Shutdown { resp: Option<Sender<()>> },
}

struct EngineHandleInner {
    tx: Sender<EngineMessage>,
    snapshot: Arc<ArcSwap<EngineSnapshot>>,
    event_listener: EventListener,
}

pub struct EngineHandle {
    inner: Arc<EngineHandleInner>,
}

impl EngineHandle {
    pub fn new(engine: Engine) -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();

        let snapshot = Arc::new(ArcSwap::from_pointee(engine.snapshot()));

        let event_listener = engine.event_listener();

        thread::Builder::new()
            .name("rd_engine".to_string())
            .spawn({
                let snapshot = Arc::clone(&snapshot);
                move || engine.run(rx, snapshot)
            })
            .expect("Failed to spawn engine thread");

        Self { inner: Arc::new(EngineHandleInner { tx, snapshot, event_listener }) }
    }

    pub fn execute(&self, command: Command) -> anyhow::Result<()> {
        let rx = self.execute_async(command)?;
        rx.recv().map_err(|_| anyhow::anyhow!("Engine thread stopped"))?
    }

    pub fn execute_async(&self, command: Command) -> anyhow::Result<Receiver<anyhow::Result<()>>> {
        let (resp_tx, resp_rx) = crossbeam_channel::bounded(1);
        self.inner
            .tx
            .send(EngineMessage::Command { command, resp: Some(resp_tx) })
            .map_err(|_| anyhow::anyhow!("Engine thread stopped"))?;
        Ok(resp_rx)
    }

    pub fn try_execute(&self, command: Command) -> anyhow::Result<()> {
        self.inner
            .tx
            .send(EngineMessage::Command { command, resp: None })
            .map_err(|_| anyhow::anyhow!("Engine thread stopped"))
    }

    pub fn snapshot(&self) -> EngineSnapshot {
        self.inner.snapshot.load().as_ref().clone()
    }

    pub fn event_listener(&self) -> EventListener {
        self.inner.event_listener.clone()
    }

    pub fn shutdown_async(&self) -> anyhow::Result<Receiver<()>> {
        let (resp_tx, resp_rx) = crossbeam_channel::bounded(1);
        self.inner
            .tx
            .send(EngineMessage::Shutdown { resp: Some(resp_tx) })
            .map_err(|_| anyhow::anyhow!("Engine thread stopped"))?;
        Ok(resp_rx)
    }
}

impl Clone for EngineHandle {
    fn clone(&self) -> Self {
        Self { inner: Arc::clone(&self.inner) }
    }
}

#[derive(Default)]
pub struct EngineSnapshot {
    showfile_path: Option<PathBuf>,
    patch: Arc<Patch>,
    output: Arc<OutputDefinition>,
    triggers: Arc<TriggersDefinition>,
    objects: Arc<Objects>,
    selection: Arc<Selection>,
    highlight: bool,
}

impl EngineSnapshot {
    pub fn showfile_path(&self) -> Option<&Path> {
        self.showfile_path.as_deref()
    }

    pub fn patch(&self) -> &Patch {
        &self.patch
    }

    pub fn output(&self) -> &OutputDefinition {
        &self.output
    }

    pub fn triggers(&self) -> &TriggersDefinition {
        &self.triggers
    }

    pub fn objects(&self) -> &Objects {
        &self.objects
    }

    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    pub fn highlight(&self) -> bool {
        self.highlight
    }
}

impl Clone for EngineSnapshot {
    fn clone(&self) -> Self {
        Self {
            showfile_path: self.showfile_path.clone(),
            patch: Arc::clone(&self.patch),
            output: Arc::clone(&self.output),
            triggers: Arc::clone(&self.triggers),
            objects: Arc::clone(&self.objects),
            selection: Arc::clone(&self.selection),
            highlight: self.highlight,
        }
    }
}
