use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use anyhow::Context as _;
use zeevonk::{
    Zeevonk,
    project::{FixtureId, Patch, ProjectFile as ZeevonkProjectFile, Stage},
};

mod command;
mod compositor;
mod config;
mod effect;
mod event;
mod object;
mod output;
mod parameter;
mod programmer;

pub use command::*;
pub use compositor::*;
pub use config::*;
pub use effect::*;
pub use event::*;
pub use object::*;
pub use output::*;
pub use parameter::*;
pub use programmer::*;

pub use zeevonk as zv;

pub struct RadiantEngine {
    showfile_path: Option<PathBuf>,

    config: Arc<Config>,
    objects: Arc<ObjectRegistry>,
    programmer: Arc<Programmer>,
    output_agent: Arc<OutputAgent>,
    effect_agent: Arc<EffectAgent>,
    zeevonk: Arc<Zeevonk>,

    selection: Arc<RwLock<Vec<FixtureId>>>,
    highlight: Arc<RwLock<bool>>,

    event_tx: crossbeam_channel::Sender<Event>,
    event_rx: crossbeam_channel::Receiver<Event>,
}

impl RadiantEngine {
    pub fn new(showfile_path: Option<PathBuf>) -> anyhow::Result<Self> {
        fn load_objects<T>(obj_registry: &mut ObjectRegistry, file: PathBuf) -> anyhow::Result<()>
        where
            T: Object + serde::de::DeserializeOwned + 'static,
        {
            if !file.exists() {
                return Ok(());
            }

            let content = fs::read_to_string(&file)
                .with_context(|| format!("Failed to read object file: {}", file.display()))?;
            let objects: Vec<T> = serde_json::from_str(&content).with_context(|| {
                format!("Failed to deserialize objects from: {}", file.display())
            })?;

            for object in objects {
                obj_registry.insert(object);
            }
            Ok(())
        }

        let mut config = Config::default();
        let mut objects = ObjectRegistry::default();
        let mut zv_project_file = ZeevonkProjectFile::default();

        if let Some(path) = &showfile_path {
            // Load config
            config = serde_json::from_reader(
                fs::File::open(path.join("config.json")).context("failed to open config file")?,
            )
            .context("failed to load config file")?;

            // Load objects.
            load_objects::<Effect>(&mut objects, path.join("obj/effects.json"))
                .context("Failed to load effects object file")?;
            load_objects::<Group>(&mut objects, path.join("obj/groups.json"))
                .context("Failed to load groups object file")?;
            load_objects::<CueList>(&mut objects, path.join("obj/cue_lists.json"))
                .context("Failed to load cue lists object file")?;
            load_objects::<LayoutPage>(&mut objects, path.join("obj/layout_pages.json"))
                .context("Failed to load layout pages object file")?;
            load_objects::<ExecutorPage>(&mut objects, path.join("obj/executor_pages.json"))
                .context("Failed to load executor pages object file")?;

            // Load zeevonk project file.
            zv_project_file = ZeevonkProjectFile::load_from_folder(&path.join("zv/"))
                .with_context(|| {
                    format!(
                        "Failed to load Zeevonk project file from {}",
                        path.join("zv/").display()
                    )
                })?;
        }

        let config = Arc::new(config);
        let objects = Arc::new(objects);
        let effect_agent = Arc::new(EffectAgent::new(Arc::clone(&objects)));
        let programmer = Arc::new(Programmer::new());
        let zeevonk =
            Arc::new(Zeevonk::new(zv_project_file).context("Failed to initialize Zeevonk engine")?);
        let selection = Arc::new(RwLock::new(Vec::new()));
        let highlight = Arc::new(RwLock::new(false));
        let output_agent = Arc::new(OutputAgent::new(
            Arc::clone(&objects),
            Arc::clone(&programmer),
            Arc::clone(&effect_agent),
            Arc::clone(&zeevonk),
            Arc::clone(&selection),
            Arc::clone(&highlight),
        ));

        let (event_tx, event_rx) = crossbeam_channel::unbounded();

        Ok(Self {
            showfile_path,

            config,
            objects,
            programmer,
            output_agent,
            effect_agent,
            zeevonk,

            selection,
            highlight,

            event_rx,
            event_tx,
        })
    }

    pub fn start(&self) {
        self.zeevonk.start();
        self.output_agent().start();
    }

    pub fn try_exec(&self, command: impl Into<Command>) -> anyhow::Result<()> {
        command::execute(command.into(), self, true)
    }

    pub fn exec(&self, command: impl Into<Command>) {
        if let Err(err) = self.try_exec(command) {
            log::error!("Failed to execute command: {err}");
        }
    }

    pub fn exec_without_emit(&self, command: impl Into<Command>) {
        if let Err(err) = command::execute(command.into(), self, false) {
            log::error!("Failed to execute command: {err}");
        }
    }

    pub fn showfile_path(&self) -> Option<&Path> {
        self.showfile_path.as_deref()
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn objects(&self) -> &ObjectRegistry {
        &self.objects
    }

    pub fn programmer(&self) -> &Programmer {
        &self.programmer
    }

    pub fn output_agent(&self) -> &OutputAgent {
        &self.output_agent
    }

    pub fn effect_agent(&self) -> &EffectAgent {
        &self.effect_agent
    }

    pub fn patch(&self) -> &Patch {
        &self.zeevonk.project().file().patch
    }

    pub fn stage(&self) -> &Stage {
        &self.zeevonk.project().stage()
    }

    pub fn selection(&self) -> Vec<FixtureId> {
        self.selection.read().unwrap().clone()
    }

    pub fn highlight(&self) -> bool {
        self.highlight.read().unwrap().clone()
    }

    pub fn event_rx(&self) -> &crossbeam_channel::Receiver<Event> {
        &self.event_rx
    }

    pub(crate) fn emit(&self, event: Event) {
        if let Err(err) = self.event_tx.send(event) {
            log::error!("engine event channel closed: {err}");
        }
    }
}
