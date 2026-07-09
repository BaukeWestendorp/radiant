use std::path::PathBuf;

#[derive(Debug)]
pub enum Event {
    HighlightChanged { highlight: bool },

    Saved { path: PathBuf },
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
