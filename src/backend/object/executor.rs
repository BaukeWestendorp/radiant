use std::str::FromStr;

use crate::backend::{Cue, Sequence, SequenceId, Show};

crate::define_object_id!(ExecutorId);

/// An executor controls how a sequence will be activated and terminated.
#[derive(Debug, Clone, PartialEq)]
pub struct Executor {
    id: ExecutorId,
    pub name: String,
    pub(in crate::backend) activation_mode: ActivationMode,
    pub(in crate::backend) termination_mode: TerminationMode,
    pub(in crate::backend) sequence_id: Option<SequenceId>,
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

    pub fn id(&self) -> ExecutorId {
        self.id
    }

    pub fn activation_mode(&self) -> ActivationMode {
        self.activation_mode
    }

    pub fn termination_mode(&self) -> TerminationMode {
        self.termination_mode
    }

    pub fn sequence_id(&self) -> Option<&SequenceId> {
        self.sequence_id.as_ref()
    }

    /// Gets a reference to the [Sequence] this executor is linked to.
    pub fn sequence<'a>(&self, show: &'a Show) -> Option<&'a Sequence> {
        let sequence_id = self.sequence_id?;
        show.sequence(sequence_id).or_else(move || {
            log::warn!("Sequence with id {} not found", sequence_id);
            None
        })
    }

    /// Sets the index indicating the active cue.
    pub fn set_active_cue_index(&mut self, index: Option<usize>, show: &Show) {
        let index = match index {
            Some(index) => index,
            None => {
                self.active_cue_index = None;
                return;
            }
        };

        let Some(sequence) = self.sequence(show) else { return };

        if index > sequence.len() - 1 {
            self.active_cue_index = Some(sequence.len() - 1);
        } else {
            self.active_cue_index = Some(index);
        }
    }

    /// Gets a reference to the [Cue] that is currently activated by the executor.
    pub fn get_active_cue<'a>(&self, show: &'a Show) -> Option<&'a Cue> {
        let index = self.active_cue_index?;
        let cue_id = self.sequence(show)?.cues().get(index)?;
        show.cue(*cue_id)
    }

    pub fn manage_state(&mut self, show: &Show) {
        match self.activation_mode {
            ActivationMode::Instant => self.go(show),
        }

        match self.termination_mode {
            TerminationMode::Never => {}
        }
    }

    pub fn go(&mut self, show: &Show) {
        let current_index = self.active_cue_index.unwrap_or_default();
        self.set_active_cue_index(Some(current_index + 1), show);
    }
}

/// Determines how an [Executor] is activated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ActivationMode {
    #[default]
    Instant,
}

impl FromStr for ActivationMode {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "instant" => Ok(Self::Instant),
            other => eyre::bail!("invalid activation mode: '{other}'"),
        }
    }
}

/// Determines how an [Executor] is terminated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TerminationMode {
    #[default]
    Never,
}

impl FromStr for TerminationMode {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "never" => Ok(Self::Never),
            other => eyre::bail!("invalid termination mode: '{other}'"),
        }
    }
}
