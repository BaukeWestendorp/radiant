use std::{
    path::{Path, PathBuf},
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use arc_swap::ArcSwap;
use flume::{Receiver, Sender};

use crate::{
    Project,
    cmd::Command,
    event::{Event, EventListener},
    object::Objects,
    output::OutputAgent,
    patch::Patch,
    pipeline::Pipeline,
    programmer::Programmer,
    selection::Selection,
    trigger::{Trigger, TriggersAgent},
};

pub struct Engine {
    showfile_path: Option<PathBuf>,

    pub(crate) patch: Arc<Patch>,
    pub(crate) objects: Arc<Objects>,
    pub(crate) programmer: Arc<Programmer>,
    pub(crate) pipeline: Arc<Pipeline>,
    pub(crate) selection: Arc<Selection>,
    pub(crate) highlight: bool,

    pub(crate) triggers_agent: TriggersAgent,
    pub(crate) output_agent: OutputAgent,

    event_tx: flume::Sender<Event>,
    event_listener: EventListener,
    event_buffer: Vec<Event>,
}

impl Engine {
    pub fn new(project: Project) -> anyhow::Result<Self> {
        let patch = Patch::new(project.patch().clone(), project.gdtfs().clone())?;
        let objects = project.objects().clone();
        let pipeline = Pipeline::new(&patch);

        let (event_tx, event_rx) = flume::unbounded();
        let event_listener = EventListener::new(event_rx);

        let output_agent = OutputAgent::new(project.output().clone())?;
        let triggers_agent = TriggersAgent::new(project.triggers().clone())?;

        let engine = Self {
            showfile_path: project.path().map(|p| p.to_path_buf()),

            patch: Arc::new(patch),
            objects: Arc::new(objects),
            selection: Arc::new(Selection::new()),
            programmer: Arc::new(Programmer::new()),
            pipeline: Arc::new(pipeline),
            highlight: false,

            event_tx,
            event_listener,
            event_buffer: Vec::new(),

            output_agent,
            triggers_agent,
        };

        Ok(engine)
    }

    pub fn showfile_path(&self) -> Option<&Path> {
        self.showfile_path.as_deref()
    }

    pub fn patch(&self) -> &Patch {
        &self.patch
    }

    pub fn triggers_agent(&self) -> &TriggersAgent {
        &self.triggers_agent
    }

    pub fn output_agent(&self) -> &OutputAgent {
        &self.output_agent
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
            patch: Arc::clone(&self.patch),
            objects: Arc::clone(&self.objects),
            programmer: Arc::clone(&self.programmer),
            pipeline: Arc::clone(&self.pipeline),
            selection: Arc::clone(&self.selection),
            highlight: self.highlight,
        }
    }

    pub(crate) fn emit(&mut self, event: Event) {
        self.event_buffer.push(event);
    }

    fn start(mut self, rx: Receiver<EngineMessage>, snapshot_store: Arc<ArcSwap<EngineSnapshot>>) {
        log::debug!("Starting Radiant Engine...");

        const INTERVAL: Duration = Duration::new(0, ((1_000_000_000_f64 / 60.0).round()) as u32);

        self.output_agent.start();

        let mut next_tick = Instant::now() + INTERVAL;
        let mut running = true;

        let mut started = false;
        while running {
            if !started {
                log::info!("Started Radiant Engine");
                started = true;
            }

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
                    Err(flume::RecvTimeoutError::Timeout) => {}
                    Err(flume::RecvTimeoutError::Disconnected) => {
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

        self.output_agent.stop();

        log::info!("Stopped Radiant Engine");
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
        let mut snapshot_dirty = false;
        for trigger in self.triggers_agent.drain() {
            match trigger {
                Trigger::ExecutorMaster { executor_id, value } => {
                    self.execute(Command::ExecutorSetMaster { executor_id, value })
                        .map_err(|err| log::error!("{err}"))
                        .ok();
                    snapshot_dirty = true;
                }
                Trigger::ExecutorButton { executor_id, button, pressed } => {
                    self.execute(Command::ExecutorButton { executor_id, button, pressed })
                        .map_err(|err| log::error!("{err}"))
                        .ok();
                    snapshot_dirty = true;
                }
                Trigger::EncoderSetValue { encoder_ix, value } => {
                    self.execute(Command::EncoderSetValue { encoder_ix, value })
                        .map_err(|err| log::error!("{err}"))
                        .ok();
                }
            }
        }

        self.resolve_pipeline();

        if snapshot_dirty {
            snapshot_store.store(Arc::new(self.generate_snapshot()));
        }

        for event in self.event_buffer.drain(..) {
            let _ = self.event_tx.send(event);
        }

        self.output_agent.update(self.pipeline.multiverse().clone());
    }

    fn resolve_pipeline(&mut self) {
        let pipeline = Arc::make_mut(&mut self.pipeline);

        let highlighted_fixtures =
            self.highlight.then(|| self.selection.fixture_ids().to_vec()).unwrap_or_default();
        if let Err(err) = pipeline.resolve_attributes(
            &self.objects,
            &self.patch,
            &self.programmer,
            highlighted_fixtures,
        ) {
            log::error!("Failed to resolve attribute values: {err}");
            return;
        };

        pipeline.resolve_dmx();

        self.emit(Event::PipelineResolved);
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
        let (tx, rx) = flume::unbounded();

        let snapshot = Arc::new(ArcSwap::from_pointee(engine.generate_snapshot()));

        let event_listener = engine.event_listener();

        thread::Builder::new()
            .name("rd_engine".to_string())
            .spawn({
                let snapshot = Arc::clone(&snapshot);
                move || engine.start(rx, snapshot)
            })
            .expect("Failed to spawn engine thread");

        Self { inner: Arc::new(EngineHandleInner { tx, snapshot, event_listener }) }
    }

    pub fn execute(&self, command: Command) -> anyhow::Result<()> {
        let (resp_tx, resp_rx) = flume::bounded(1);
        self.inner
            .tx
            .send(EngineMessage::Command { command, resp: Some(resp_tx) })
            .map_err(|_| anyhow::anyhow!("Engine thread stopped"))?;
        resp_rx.recv().map_err(|_| anyhow::anyhow!("Engine thread stopped"))?
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

    pub fn shutdown(&self) -> anyhow::Result<()> {
        let (resp_tx, resp_rx) = flume::bounded(1);
        self.inner
            .tx
            .send(EngineMessage::Shutdown { resp: Some(resp_tx) })
            .map_err(|_| anyhow::anyhow!("Engine thread stopped"))?;
        resp_rx.recv().map_err(|_| anyhow::anyhow!("Engine thread stopped"))?;
        Ok(())
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
    patch: Arc<Patch>,
    objects: Arc<Objects>,
    programmer: Arc<Programmer>,
    pipeline: Arc<Pipeline>,
    selection: Arc<Selection>,
    highlight: bool,
}

impl EngineSnapshot {
    pub fn showfile_path(&self) -> Option<&Path> {
        self.showfile_path.as_deref()
    }

    pub fn patch(&self) -> Arc<Patch> {
        Arc::clone(&self.patch)
    }

    pub fn objects(&self) -> Arc<Objects> {
        Arc::clone(&self.objects)
    }

    pub fn programmer(&self) -> Arc<Programmer> {
        Arc::clone(&self.programmer)
    }

    pub fn pipeline(&self) -> Arc<Pipeline> {
        Arc::clone(&self.pipeline)
    }

    pub fn selection(&self) -> Arc<Selection> {
        Arc::clone(&self.selection)
    }

    pub fn highlight(&self) -> bool {
        self.highlight
    }
}
