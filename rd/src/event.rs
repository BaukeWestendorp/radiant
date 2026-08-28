use std::path::PathBuf;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    ProjectLoaded,
    ProjectUnloaded,
    ProjectReloaded,

    Saved { path: PathBuf },

    HighlightChanged { highlight: bool },
}

pub struct Events {
    rx: flume::Receiver<Event>,
}

impl Events {
    pub(crate) fn new(rx: flume::Receiver<Event>) -> Self {
        Self { rx }
    }
}

impl std::ops::Deref for Events {
    type Target = flume::Receiver<Event>;

    fn deref(&self) -> &Self::Target {
        &self.rx
    }
}

impl Clone for Events {
    fn clone(&self) -> Self {
        Self { rx: flume::Receiver::clone(&self.rx) }
    }
}
