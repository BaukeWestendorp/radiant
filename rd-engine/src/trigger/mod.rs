use crate::object::{ExecutorButton, ExecutorId};

mod agent;
mod definition;

pub use agent::*;
pub use definition::*;

pub enum Trigger {
    ExecutorMaster { executor_id: ExecutorId, value: f32 },
    ExecutorButton { executor_id: ExecutorId, button: ExecutorButton, pressed: bool },
    EncoderSetValue { encoder_ix: usize, value: f32 },
}
