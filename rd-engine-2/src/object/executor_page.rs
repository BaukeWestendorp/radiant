use std::fmt;

use crate::{Object, ObjectId, Slot};

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
    content: Option<ExecutorContent>,
    enabled: bool,
    master: f32,

    #[serde(skip)]
    flash_restore_master: Option<f32>,
}

impl Executor {
    pub fn content(&self) -> Option<&ExecutorContent> {
        self.content.as_ref()
    }

    pub fn content_mut(&mut self) -> &mut Option<ExecutorContent> {
        &mut self.content
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

    pub(crate) fn set_master(&mut self, master: f32) {
        self.master = master.clamp(0.0, 1.0)
    }

    pub(crate) fn flash_master_press(&mut self) {
        if self.flash_restore_master.is_none() {
            self.flash_restore_master = Some(self.master());
        }
        self.set_master(1.0);
    }

    pub(crate) fn flash_master_release(&mut self) {
        if let Some(prev) = self.flash_restore_master.take() {
            self.set_master(prev);
        }
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
    CueList {
        cue_list: ObjectId,
        cue_index: usize,
        priority: u32,
        merge_mode: MergeMode,
        start_from_previous_cue: bool,
        master_controls_enabled: bool,
        reset_to_start_on_disable: bool,
        button1: ExecutorButtonAction,
        button2: ExecutorButtonAction,
        button3: ExecutorButtonAction,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MergeMode {
    /// Highest Takes Precedence
    #[serde(rename = "HTP")]
    Htp,
    /// Latest Takes Precedence
    #[serde(rename = "LTP")]
    Ltp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExecutorId {
    pub page: ObjectId,
    pub executor: Slot,
}

impl ExecutorId {
    pub fn new(page: ObjectId, executor: Slot) -> Self {
        Self { page, executor }
    }
}

impl fmt::Display for ExecutorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.page, self.executor)
    }
}
