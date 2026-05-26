use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use zeevonk::project::Stage;

use crate::{Config, Objects, Selection};

#[derive(Debug, Clone, Default)]
pub struct EngineSnapshot {
    pub(crate) showfile_path: Option<PathBuf>,

    pub(crate) stage: Arc<Stage>,
    pub(crate) config: Arc<Config>,
    pub(crate) objects: Arc<Objects>,
    pub(crate) selection: Arc<Selection>,
}

impl EngineSnapshot {
    pub fn showfile_path(&self) -> Option<&Path> {
        self.showfile_path.as_deref()
    }

    pub fn stage(&self) -> &Stage {
        &self.stage
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn objects(&self) -> &Objects {
        &self.objects
    }

    pub fn selection(&self) -> &Selection {
        &self.selection
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotListener {
    rx: crossbeam_channel::Receiver<Arc<EngineSnapshot>>,
}

impl SnapshotListener {
    pub fn recv(&self) -> Option<Arc<EngineSnapshot>> {
        self.rx.recv().ok()
    }

    pub fn try_recv(&self) -> Option<Arc<EngineSnapshot>> {
        self.rx.try_recv().ok()
    }
}
