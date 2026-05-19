use crate::{Object, ObjectId, ObjectKind, SlotId, builtin::BuiltinEffect};

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Effect {
    id: ObjectId,
    slot_id: SlotId,
    name: String,

    kind: EffectKind,
}

impl Effect {
    pub fn kind(&self) -> &EffectKind {
        &self.kind
    }
}

impl Object for Effect {
    fn kind() -> ObjectKind {
        ObjectKind::Effect
    }

    fn id(&self) -> ObjectId {
        self.id
    }

    fn slot_id(&self) -> SlotId {
        self.slot_id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum EffectKind {
    Builtin(BuiltinEffect),
}
