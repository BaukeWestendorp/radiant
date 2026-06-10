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
    programmer::Programmer,
    selection::Selection,
    trigger::{Triggers, TriggersDefinition},
};

pub struct Engine {
    showfile_path: Option<PathBuf>,

    pub(crate) patch: Patch,
    pub(crate) output: Output,
    pub(crate) triggers: Triggers,
    pub(crate) objects: Objects,
    pub(crate) programmer: Programmer,
    pub(crate) pipeline: Pipeline,
    pub(crate) selection: Selection,
    pub(crate) highlight: bool,

    event_tx: crossbeam_channel::Sender<Event>,
    event_listener: EventListener,
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

        let (event_tx, event_rx) = crossbeam_channel::unbounded();
        let event_listener = EventListener::new(event_rx);

        let pipeline = Pipeline::new(&patch);

        let engine = Self {
            showfile_path: project.file().map(|p| p.path().clone()),

            patch,
            output,
            triggers,
            objects,

            event_tx,
            event_listener,

            selection: Selection::new(),
            programmer: Programmer::new(),
            highlight: false,

            pipeline,
        };

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

    pub fn programmer(&self) -> &Programmer {
        &self.programmer
    }

    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    pub fn highlight(&self) -> bool {
        self.highlight
    }

    pub fn event_listener(&self) -> EventListener {
        self.event_listener.clone()
    }

    pub fn execute(&mut self, command: Command) -> anyhow::Result<()> {
        command.execute(self)
    }

    pub fn generate_snapshot(&self) -> EngineSnapshot {
        EngineSnapshot {
            showfile_path: self.showfile_path.clone(),
            patch: self.patch.clone(),
            output: self.output.definition().clone(),
            triggers: self.triggers.definition().clone(),
            objects: self.objects.clone(),
            programmer: self.programmer.clone(),
            pipeline: self.pipeline.clone(),
            selection: self.selection.clone(),
            highlight: self.highlight,
        }
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
                match rx.recv_timeout(next_tick - now) {
                    Ok(msg) => {
                        running = self.handle_message(msg, &snapshot_store);
                    }
                    Err(crossbeam_channel::RecvTimeoutError::Timeout) => {}
                    Err(crossbeam_channel::RecvTimeoutError::Disconnected) => {
                        running = false;
                    }
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
                self.resolve_pipeline();
                snapshot_store.store(Arc::new(self.generate_snapshot()));
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
        let snapshot_dirty = !commands.is_empty();

        for command in commands {
            if let Err(err) = self.execute(command) {
                log::error!("Failed to execute command: {err}");
            }
        }

        self.resolve_pipeline();

        if snapshot_dirty {
            snapshot_store.store(Arc::new(self.generate_snapshot()));
        }

        self.output.update(self.pipeline.multiverse().clone());
    }

    fn resolve_pipeline(&mut self) {
        let highlighted_fixtures =
            self.highlight.then(|| self.selection.fixture_ids().to_vec()).unwrap_or_default();
        if let Err(err) = self.pipeline.resolve_attributes(
            &self.objects,
            &self.patch,
            &self.programmer,
            highlighted_fixtures,
        ) {
            log::error!("Failed to resolve attribute values: {err}");
            return;
        };

        self.pipeline.resolve_dmx();
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

        let snapshot = Arc::new(ArcSwap::from_pointee(engine.generate_snapshot()));

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

    pub fn snapshot(&self) -> Arc<EngineSnapshot> {
        self.inner.snapshot.load_full()
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

#[derive(Clone, Default)]
pub struct EngineSnapshot {
    showfile_path: Option<PathBuf>,
    patch: Patch,
    output: OutputDefinition,
    triggers: TriggersDefinition,
    objects: Objects,
    programmer: Programmer,
    pipeline: Pipeline,
    selection: Selection,
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

    pub fn programmer(&self) -> &Programmer {
        &self.programmer
    }

    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    pub fn highlight(&self) -> bool {
        self.highlight
    }
}
