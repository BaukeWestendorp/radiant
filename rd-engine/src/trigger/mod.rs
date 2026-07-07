use crate::object::{ExecutorButton, ExecutorId};

mod definition;
mod service;

pub use definition::*;
pub use service::*;

pub enum Trigger {
    ExecutorMaster { executor_id: ExecutorId, value: f32 },
    ExecutorButton { executor_id: ExecutorId, button: ExecutorButton, pressed: bool },
    EncoderSetValue { encoder_ix: usize, value: f32 },
}
