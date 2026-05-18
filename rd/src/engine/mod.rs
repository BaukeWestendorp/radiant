use std::{
    path::{Path, PathBuf},
    sync::Arc,
    thread,
};

use gpui::{App, AppContext as _, BorrowAppContext, Entity};
use zeevonk::{Zeevonk, project::ProjectFile as ZeevonkProjectFile};

mod command;
mod compositor;
mod effect;
mod lua;
mod object;
mod output;
mod parameter;
mod programmer;
mod selection;

pub use command::*;
pub use compositor::*;
pub use effect::*;
pub use object::*;
pub use output::*;
pub use parameter::*;
pub use programmer::*;
pub use selection::*;

pub struct Engine {
    showfile_path: Option<PathBuf>,

    objects: Arc<ObjectRegistry>,
    programmer: Arc<Programmer>,
    output_agent: Arc<OutputAgent>,
    effect_agent: Arc<EffectAgent>,
    zeevonk: Arc<Zeevonk>,

    selection: Entity<Selection>,
}

impl Engine {
    pub fn new(showfile_path: Option<PathBuf>, cx: &mut App) -> anyhow::Result<Self> {
        let objects = Arc::new(ObjectRegistry::new());
        let effect_agent = Arc::new(EffectAgent::new(Arc::clone(&objects), showfile_path.clone()));
        let programmer = Arc::new(Programmer::new());
        let zeevonk = Arc::new(Zeevonk::new(ZeevonkProjectFile::default())?);
        let output_agent = Arc::new(OutputAgent::new(
            Arc::clone(&objects),
            Arc::clone(&programmer),
            Arc::clone(&effect_agent),
            Arc::clone(&zeevonk),
        ));
        let selection = cx.new(|cx| Selection::new(cx));

        let mut engine = Self {
            showfile_path: showfile_path.clone(),
            objects,
            programmer,
            output_agent,
            effect_agent,
            zeevonk,
            selection,
        };

        if let Some(showfile_path) = showfile_path {
            engine.execute_and_log_err(Command::Load(showfile_path), cx);
        }

        Ok(engine)
    }

    pub fn start(&self) {
        self.zeevonk().start();
        self.output_agent().start();

        let _ = thread::Builder::new().name("rd_engine_main".to_string()).spawn({
            let output_agent = Arc::clone(&self.output_agent);
            move || {
                let _ = output_agent;
            }
        });
    }

    pub fn execute(&mut self, command: impl Into<Command>, cx: &mut App) -> anyhow::Result<()> {
        command::execute(command.into(), self, cx)
    }

    pub fn execute_and_log_err(&mut self, command: impl Into<Command>, cx: &mut App) {
        if let Err(err) = command::execute(command.into(), self, cx) {
            log::error!("Failed to execute command: {err}");
        }
    }

    pub fn execute_on_global(command: impl Into<Command>, cx: &mut App) -> anyhow::Result<()> {
        cx.update_global::<Self, _>(|engine, cx| command::execute(command.into(), engine, cx))
    }

    pub fn execute_on_global_and_log_err(command: impl Into<Command>, cx: &mut App) {
        if let Err(err) = Self::execute_on_global(command, cx) {
            log::error!("Failed to execute command: {err}");
        }
    }

    pub fn showfile_path(&self) -> Option<&Path> {
        self.showfile_path.as_deref()
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

    pub fn zeevonk(&self) -> &Zeevonk {
        &self.zeevonk
    }

    pub fn selection<'a>(&'a self, cx: &'a App) -> &'a Selection {
        self.selection.read(cx)
    }

    pub fn selection_entity(&self) -> &Entity<Selection> {
        &self.selection
    }
}

impl gpui::Global for Engine {}
