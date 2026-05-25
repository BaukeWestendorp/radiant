use std::{
    collections::VecDeque,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context as _;
use zeevonk::{Zeevonk, project::ProjectFile as ZeevonkProjectFile};

use crate::pipeline::Pipeline;

mod cmd;
mod config;
mod object;
mod pipeline;

pub use cmd::*;
pub use config::*;
pub use object::*;

pub struct Engine {
    showfile_path: Option<PathBuf>,

    config: Config,
    objects: Objects,
    pipeline: Pipeline,

    command_queue: VecDeque<Command>,

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

        Ok(Self {
            showfile_path,

            pipeline: Pipeline::new(&config).context("failed to build pipeline")?,
            objects,
            config,

            command_queue: VecDeque::new(),

            zeevonk: Zeevonk::new(zv_project_file)
                .context("failed to initialize zeevonk engine")?,
        })
    }

    pub fn start(&mut self) {
        self.zeevonk.start();

        loop {
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0 / 60.0));

            let trigger_commands = match self.pipeline.resolve_triggers() {
                Ok(commands) => commands,
                Err(err) => {
                    log::error!("failed to resolve triggers: {err}");
                    continue;
                }
            };

            if let Err(err) = self.exec_commands(trigger_commands) {
                log::error!("failed to execute command: {err}");
            };

            let output = match self.pipeline.compose(&self.objects, self.zeevonk.project().stage())
            {
                Ok(output) => output,
                Err(err) => {
                    log::error!("failed to composite: {err}");
                    continue;
                }
            };

            self.zeevonk.clear_attribute_values();
            self.zeevonk.set_attribute_values(output);

            while let Some(queued_command) = self.command_queue.pop_front() {
                if let Err(err) = self.exec_command(queued_command) {
                    log::error!("failed to execute queued command: {err}");
                }
            }
        }
    }

    pub fn showfile_path(&self) -> Option<&Path> {
        self.showfile_path.as_deref()
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn set_config(&mut self, config: Config) -> anyhow::Result<()> {
        self.pipeline = Pipeline::new(&config).context("failed to build pipeline")?;
        self.config = config;
        Ok(())
    }

    pub fn groups(&self) -> &ObjectCollection<Group> {
        &self.objects.groups
    }

    pub fn executor_pages(&self) -> &ObjectCollection<ExecutorPage> {
        &self.objects.executor_pages
    }

    pub fn queue_command(&mut self, command: Command) {
        self.command_queue.push_back(command);
    }

    fn exec_commands(&mut self, commands: impl IntoIterator<Item = Command>) -> anyhow::Result<()> {
        for command in commands {
            self.exec_command(command)?
        }
        Ok(())
    }

    fn exec_command(&mut self, command: Command) -> anyhow::Result<()> {
        match command {
            Command::ExecutorSetMaster { executor_id, value } => {
                let page = self.objects.executor_pages.get_by_object_id_mut(&executor_id.page)?;
                let executor = page.executor_mut(executor_id.executor)?;

                let prev_master = executor.master();
                executor.set_master(value);
                let new_master = executor.master();

                if let Some(ExecutorContent::CueList { master_controls_enabled, .. }) =
                    executor.content()
                {
                    if *master_controls_enabled {
                        if prev_master == 0.0 && new_master > 0.0 {
                            executor.set_enabled(true);
                        }
                        if prev_master > 0.0 && new_master == 0.0 {
                            executor.set_enabled(false);
                        }
                    }
                }

                reset_cue_list_to_start_if_disabled(executor);
            }
            Command::ExecutorToggleEnabled { executor_id } => {
                let page = self.objects.executor_pages.get_by_object_id_mut(&executor_id.page)?;
                let executor = page.executor_mut(executor_id.executor)?;

                executor.set_enabled(!executor.enabled());
                reset_cue_list_to_start_if_disabled(executor);
            }
            Command::ExecutorSetEnabled { executor_id, value } => {
                let page = self.objects.executor_pages.get_by_object_id_mut(&executor_id.page)?;
                let executor = page.executor_mut(executor_id.executor)?;
                executor.set_enabled(value);
                reset_cue_list_to_start_if_disabled(executor);
            }
            Command::ExecutorButton { executor_id, button, pressed } => {
                let page = self.objects.executor_pages.get_by_object_id_mut(&executor_id.page)?;
                let executor = page.executor_mut(executor_id.executor)?;

                let action = match executor.content() {
                    Some(ExecutorContent::CueList { button1, button2, button3, .. }) => {
                        match button {
                            ExecutorButton::Button1 => *button1,
                            ExecutorButton::Button2 => *button2,
                            ExecutorButton::Button3 => *button3,
                        }
                    }
                    None => return Ok(()),
                };

                match action {
                    ExecutorButtonAction::ToggleEnabled => {
                        if pressed {
                            executor.set_enabled(!executor.enabled());
                            reset_cue_list_to_start_if_disabled(executor);
                        }
                    }
                    ExecutorButtonAction::SetEnabled { value } => {
                        if pressed {
                            executor.set_enabled(value);
                            reset_cue_list_to_start_if_disabled(executor);
                        }
                    }
                    ExecutorButtonAction::FlashMaster => {
                        if pressed {
                            executor.flash_master_press();
                        } else {
                            executor.flash_master_release();
                        }
                    }
                    ExecutorButtonAction::CueGoNext => {
                        if pressed {
                            if let Some(ExecutorContent::CueList { cue_list, cue_index, .. }) =
                                executor.content_mut().as_mut()
                            {
                                let cue_list_obj =
                                    self.objects.cue_lists.get_by_object_id(cue_list)?;
                                if *cue_index + 1 < cue_list_obj.cues().len() {
                                    *cue_index += 1;
                                }
                            }
                        }
                    }
                    ExecutorButtonAction::CueGoPrevious => {
                        if pressed {
                            if let Some(ExecutorContent::CueList { cue_index, .. }) =
                                executor.content_mut().as_mut()
                            {
                                *cue_index = cue_index.saturating_sub(1);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn reset_cue_list_to_start_if_disabled(executor: &mut Executor) {
    if executor.enabled() {
        return;
    }

    let Some(ExecutorContent::CueList { cue_index, reset_to_start_on_disable, .. }) =
        executor.content_mut().as_mut()
    else {
        return;
    };

    if *reset_to_start_on_disable {
        *cue_index = 0;
    }
}
