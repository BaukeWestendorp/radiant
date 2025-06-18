use crate::backend::{
    object::{Cue, Sequence, SequenceId},
    show::Show,
};

crate::define_object_id!(ExecutorId);

/// An executor controls how a sequence will be activated and terminated.
#[derive(Debug, Clone, PartialEq)]
pub struct Executor {
    pub(in crate::backend) id: ExecutorId,
    pub name: String,
    pub activation_mode: ActivationMode,
    pub termination_mode: TerminationMode,
    pub sequence_id: Option<SequenceId>,
    active_cue_index: Option<usize>,
}

impl Executor {
    pub fn new(id: impl Into<ExecutorId>) -> Self {
        Self {
            id: id.into(),
            name: "New Executor".to_string(),
            activation_mode: ActivationMode::default(),
            termination_mode: TerminationMode::default(),
            sequence_id: None,
            active_cue_index: None,
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

    pub fn with_active_cue_index(mut self, index: Option<usize>, show: &Show) -> Self {
        self.set_active_cue_index(index, show);
        self
    }

    pub fn sequence<'a>(&self, show: &'a Show) -> Option<&'a Sequence> {
        let sequence_id = self.sequence_id?;
        show.sequences.get(&sequence_id).or_else(move || {
            log::warn!("Sequence with id {} not found", sequence_id);
            None
        })
    }

    pub fn set_active_cue_index(&mut self, index: Option<usize>, show: &Show) {
        let index = match index {
            Some(index) => index,
            None => {
                self.active_cue_index = None;
                return;
            }
        };

        let Some(sequence) = self.sequence(show) else { return };

        if index > sequence.len() {
            self.active_cue_index = None;
        } else {
            self.active_cue_index = Some(index);
        }
    }

    pub fn get_active_cue<'a>(&self, show: &'a Show) -> Option<&'a Cue> {
        let index = self.active_cue_index?;
        self.sequence(show)?.cue(index)
    }
}

/// Determines how an [Executor] is activated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ActivationMode {
    #[default]
    Instant,
}

/// Determines how an [Executor] is terminated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TerminationMode {
    #[default]
    Never,
}
