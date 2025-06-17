use crate::backend::object::SequenceId;

crate::define_object_id!(ExecutorId);

#[derive(Debug, Clone, PartialEq)]
pub struct Executor {
    pub id: ExecutorId,
    pub name: String,
    pub activation_mode: ActivationMode,
    pub termination_mode: TerminationMode,
    pub sequence_id: Option<SequenceId>,
}

impl Executor {
    pub fn new(id: impl Into<ExecutorId>) -> Self {
        Self {
            id: id.into(),
            name: "New Executor".to_string(),
            activation_mode: ActivationMode::default(),
            termination_mode: TerminationMode::default(),
            sequence_id: None,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_activation_mode(mut self, mode: ActivationMode) -> Self {
        self.activation_mode = mode;
        self
    }

    pub fn with_termination_mode(mut self, mode: TerminationMode) -> Self {
        self.termination_mode = mode;
        self
    }

    pub fn with_sequence(mut self, sequence_id: impl Into<SequenceId>) -> Self {
        self.sequence_id = Some(sequence_id.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ActivationMode {
    #[default]
    Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TerminationMode {
    #[default]
    Never,
}
