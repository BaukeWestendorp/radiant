use crate::ExecutorId;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    ExecutorChanged(ExecutorId),
}

#[derive(Debug, Clone)]
pub struct EventListener {
    rx: crossbeam_channel::Receiver<Event>,
}

impl EventListener {
    pub(crate) fn new(rx: crossbeam_channel::Receiver<Event>) -> Self {
        Self { rx }
    }

    pub fn recv(&self) -> Option<Event> {
        self.rx.recv().ok()
    }

    pub fn try_recv(&self) -> Option<Event> {
        self.rx.try_recv().ok()
    }
}
