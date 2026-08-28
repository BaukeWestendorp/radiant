use std::path::PathBuf;

use crate::Project;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum EngineCommand {
    HighlightToggle,

    Save { path: PathBuf },
}

pub(crate) enum EngineRequest {
    Execute { command: EngineCommand, reply_tx: Option<flume::Sender<anyhow::Result<()>>> },
    LoadProject { project: Project, reply_tx: flume::Sender<anyhow::Result<()>> },
    ReplaceProject { project: Project, reply_tx: flume::Sender<anyhow::Result<()>> },
    UnloadProject { reply_tx: flume::Sender<anyhow::Result<Project>> },
    ReloadProject { reply_tx: flume::Sender<anyhow::Result<()>> },
    Stop,
}

pub struct EngineDispatcher {
    tx: flume::Sender<EngineRequest>,
}

impl EngineDispatcher {
    pub(crate) fn new(tx: flume::Sender<EngineRequest>) -> Self {
        Self { tx }
    }

    pub fn execute(&self, command: EngineCommand) {
        let _ = self.tx.send(EngineRequest::Execute { command, reply_tx: None });
    }
}

impl Default for EngineDispatcher {
    fn default() -> Self {
        let (tx, rx) = flume::unbounded();
        drop(rx);
        Self { tx }
    }
}

impl Clone for EngineDispatcher {
    fn clone(&self) -> Self {
        Self { tx: flume::Sender::clone(&self.tx) }
    }
}
