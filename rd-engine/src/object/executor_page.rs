use std::{fmt, time::Instant};

use crate::object::{Object, ObjectId, Slot};

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExecutorPage {
    id: ObjectId,
    slot: Slot,
    name: String,

    executors: [Executor; 18],
}

impl ExecutorPage {
    pub fn new(id: ObjectId, slot: Slot, name: String) -> Self {
        Self {
            id,
            slot,
            name,
            executors: [
                Executor::default(),
                Executor::default(),
                Executor::default(),
                Executor::default(),
                Executor::default(),
                Executor::default(),
                Executor::default(),
                Executor::default(),
                Executor::default(),
                Executor::default(),
                Executor::default(),
                Executor::default(),
                Executor::default(),
                Executor::default(),
                Executor::default(),
                Executor::default(),
                Executor::default(),
                Executor::default(),
            ],
        }
    }

    pub fn executors(&self) -> &[Executor; 18] {
        &self.executors
    }

    pub fn executor(&self, slot: Slot) -> anyhow::Result<&Executor> {
        self.executors
            .get(slot.as_u32() as usize - 1)
            .ok_or_else(|| anyhow::anyhow!("executor not found with slot: {}", slot))
    }

    pub(crate) fn executor_mut(&mut self, slot: Slot) -> anyhow::Result<&mut Executor> {
        self.executors
            .get_mut(slot.as_u32() as usize - 1)
            .ok_or_else(|| anyhow::anyhow!("executor not found with slot: {}", slot))
    }
}

impl Object for ExecutorPage {
    fn slot(&self) -> Slot {
        self.slot
    }

    fn id(&self) -> ObjectId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Executor {
    pub(crate) content: Option<ExecutorContent>,
    pub(crate) enabled: bool,
    pub(crate) master: f32,

    pub(crate) flash_restore_master: Option<f32>,
}

impl Executor {
    pub fn content(&self) -> Option<&ExecutorContent> {
        self.content.as_ref()
    }

    pub fn enabled(&self) -> bool {
        self.content.is_some() && self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled
    }

    pub fn master(&self) -> f32 {
        self.master.clamp(0.0, 1.0)
    }

    pub fn is_sequence_executor(&self) -> bool {
        matches!(self.content, Some(ExecutorContent::Sequence { .. }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ExecutorButton {
    Button1,
    Button2,
    Button3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ExecutorButtonAction {
    ToggleEnabled,
    SetEnabled { value: bool },
    FlashMaster,
    CueGoNext,
    CueGoPrevious,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ExecutorContent {
    Sequence(SequenceExecutorContent),
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SequenceExecutorContent {
    pub(crate) sequence: ObjectId,
    pub(crate) cue_index: usize,
    pub(crate) priority: u32,
    pub(crate) merge_mode: MergeMode,
    pub(crate) master_controls_enabled: bool,
    pub(crate) reset_to_start_on_disable: bool,
    pub(crate) button1: ExecutorButtonAction,
    pub(crate) button2: ExecutorButtonAction,
    pub(crate) button3: ExecutorButtonAction,

    #[serde(skip, default = "Instant::now")]
    pub(crate) last_activation_time: Instant,
}

impl SequenceExecutorContent {
    pub fn sequence(&self) -> ObjectId {
        self.sequence
    }

    pub fn cue_index(&self) -> usize {
        self.cue_index
    }

    pub fn priority(&self) -> u32 {
        self.priority
    }

    pub fn merge_mode(&self) -> MergeMode {
        self.merge_mode
    }

    pub fn master_controls_enabled(&self) -> bool {
        self.master_controls_enabled
    }

    pub fn reset_to_start_on_disable(&self) -> bool {
        self.reset_to_start_on_disable
    }

    pub fn button1(&self) -> ExecutorButtonAction {
        self.button1
    }

    pub fn button2(&self) -> ExecutorButtonAction {
        self.button2
    }

    pub fn button3(&self) -> ExecutorButtonAction {
        self.button3
    }

    pub(crate) fn last_activation_time(&self) -> Instant {
        self.last_activation_time
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MergeMode {
    /// Highest Takes Precedence
    Htp,
    /// Latest Takes Precedence
    Ltp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExecutorId {
    pub page: ObjectId,
    pub slot: Slot,
}

impl ExecutorId {
    pub fn new(page: ObjectId, slot: Slot) -> Self {
        Self { page, slot }
    }
}

impl fmt::Display for ExecutorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.page, self.slot)
    }
}
