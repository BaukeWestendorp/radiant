use std::path::PathBuf;

use crate::Project;

#[derive(Debug, Clone)]
#[derive(facet::Facet)]
#[facet(tag = "type")]
#[repr(C)]
pub enum Command {
    HighlightToggle,

    Save { path: PathBuf },
}

pub(crate) enum EngineRequest {
    Execute { command: Command, reply_tx: Option<flume::Sender<anyhow::Result<()>>> },
    LoadProject { project: Project, reply_tx: flume::Sender<anyhow::Result<()>> },
    ReplaceProject { project: Project, reply_tx: flume::Sender<anyhow::Result<()>> },
    UnloadProject { reply_tx: flume::Sender<anyhow::Result<Project>> },
    ReloadProject { reply_tx: flume::Sender<anyhow::Result<()>> },
    Stop,
}

pub struct Commander {
    tx: flume::Sender<EngineRequest>,
}

impl Commander {
    pub(crate) fn new(tx: flume::Sender<EngineRequest>) -> Self {
        Self { tx }
    }

    pub fn execute(&self, command: Command) {
        let _ = self.tx.send(EngineRequest::Execute { command, reply_tx: None });
    }
}

impl Default for Commander {
    fn default() -> Self {
        let (tx, rx) = flume::unbounded();
        drop(rx);
        Self { tx }
    }
}

impl Clone for Commander {
    fn clone(&self) -> Self {
        Self { tx: flume::Sender::clone(&self.tx) }
    }
}
