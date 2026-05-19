use crate::OnUpdateContext;

mod modulator;

pub use modulator::*;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum BuiltinEffect {
    Modulator { options: modulator::Options },
}

impl BuiltinEffect {
    pub fn call_on_update(&self, cx: OnUpdateContext) {
        match self {
            BuiltinEffect::Modulator { options } => modulator::on_update(cx, &options),
        }
    }
}
