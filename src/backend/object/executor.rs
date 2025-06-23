use std::str::FromStr;

use crate::backend::object::{Cue, Sequence, SequenceId};
use crate::backend::show::Show;

crate::define_object_id!(ExecutorId);

/// An executor controls how a sequence will be activated and terminated.
#[derive(Debug, Clone, PartialEq)]
pub struct Executor {
    id: ExecutorId,
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

    pub fn id(&self) -> ExecutorId {
        self.id
    }

    /// Gets a reference to the [Sequence] this executor is linked to.
    pub fn sequence<'a>(&self, show: &'a Show) -> Option<&'a Sequence> {
        let sequence_id = self.sequence_id?;
        show.sequence(&sequence_id).or_else(move || {
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

        if index > sequence.len() {
            self.active_cue_index = None;
        } else {
            self.active_cue_index = Some(index);
        }
    }

    /// Gets a reference to the [Cue] that is currently activated by the executor.
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
