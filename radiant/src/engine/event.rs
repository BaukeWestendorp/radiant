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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineEvent {
    CueFadeInProgress,
}
