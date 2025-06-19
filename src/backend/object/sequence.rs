use crate::backend::object::Cue;

crate::define_object_id!(SequenceId);

/// A sequence of [Cue]s that can be activated using an [Executor].
#[derive(Debug, Clone, PartialEq)]
pub struct Sequence {
    id: SequenceId,
    pub name: String,
    pub cues: Vec<Cue>,
}

impl Sequence {
    pub fn new(id: impl Into<SequenceId>) -> Self {
        Self { id: id.into(), name: "New Sequence".to_string(), cues: Vec::new() }
    }

    pub fn id(&self) -> SequenceId {
        self.id
    }

    pub fn len(&self) -> usize {
        self.cues.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn cue(&self, index: usize) -> Option<&Cue> {
        self.cues.get(index)
    }
}
