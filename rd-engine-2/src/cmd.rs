use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Command {
    HighlightToggle,

    Save { path: PathBuf },
}

pub struct Commander {
    tx: flume::Sender<Command>,
}

impl Commander {
    pub(crate) fn new(tx: flume::Sender<Command>) -> Self {
        Self { tx }
    }

    pub fn execute(&self, command: Command) {
        let _ = self.tx.send(command);
    }
}

impl Default for Commander {
    fn default() -> Self {
        let (tx, _rx) = flume::bounded(0);
        Self { tx }
    }
}

impl Clone for Commander {
    fn clone(&self) -> Self {
        Self { tx: flume::Sender::clone(&self.tx) }
    }
}
