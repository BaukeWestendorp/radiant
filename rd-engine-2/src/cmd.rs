use crate::{ExecutorButton, ExecutorId};

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    ExecutorSetMaster { executor_id: ExecutorId, value: f32 },
    ExecutorToggleEnabled { executor_id: ExecutorId },
    ExecutorSetEnabled { executor_id: ExecutorId, value: bool },
    ExecutorButton { executor_id: ExecutorId, button: ExecutorButton, pressed: bool },
}
