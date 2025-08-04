use std::sync::Mutex;

pub struct EventHandler {
    pending_events: Mutex<Vec<EngineEvent>>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self { pending_events: Mutex::new(Vec::new()) }
    }

    pub fn emit_event(&self, event: EngineEvent) {
        self.pending_events.lock().unwrap().push(event);
    }

    pub fn pending_events(&self) -> Vec<EngineEvent> {
        self.pending_events.lock().unwrap().clone()
    }

    pub fn drain_pending_events(&self) -> impl IntoIterator<Item = EngineEvent> {
        let mut events = self.pending_events.lock().unwrap();
        events.drain(..).collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineEvent {
    SelectionChanged,
    CueFadeInProgress,
}
