use crate::object::{ObjectId, ObjectKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    SelectionChanged,
    PipelineResolved,
    HighlightChanged { enabled: bool },
    ObjectChanged { kind: ObjectKind, object_id: ObjectId },
}

#[derive(Debug, Clone)]
pub struct EventListener {
    rx: flume::Receiver<Event>,
}

impl std::ops::Deref for EventListener {
    type Target = flume::Receiver<Event>;

    fn deref(&self) -> &Self::Target {
        &self.rx
    }
}

impl EventListener {
    pub(crate) fn new(rx: flume::Receiver<Event>) -> Self {
        Self { rx }
    }
}
