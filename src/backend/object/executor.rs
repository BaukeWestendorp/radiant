use std::str::FromStr;

use crate::backend::{Cue, Sequence, SequenceId, Show};

crate::define_object_id!(ExecutorId);

/// An executor controls how a sequence will be activated and terminated.
#[derive(Debug, Clone, PartialEq)]
pub struct Executor {
    id: ExecutorId,
    pub name: String,
    pub(in crate::backend) button_mode: ButtonMode,
    pub(in crate::backend) fader_mode: FaderMode,
    pub(in crate::backend) sequence_id: Option<SequenceId>,
    active_cue_index: Option<usize>,
}

impl Executor {
    pub fn new(id: impl Into<ExecutorId>) -> Self {
        Self {
            id: id.into(),
            name: "New Executor".to_string(),
            button_mode: ButtonMode::default(),
            fader_mode: FaderMode::default(),
            sequence_id: None,
            active_cue_index: None,
        }
    }

    pub fn id(&self) -> ExecutorId {
        self.id
    }

    pub fn button_mode(&self) -> ButtonMode {
        self.button_mode
    }

    pub fn fader_mode(&self) -> FaderMode {
        self.fader_mode
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
        match self.button_mode {
            ButtonMode::Go => self.go(show),
        }

        match self.fader_mode {
            FaderMode::Master => {}
        }
    }

    pub fn go(&mut self, show: &Show) {
        let current_index = self.active_cue_index.unwrap_or_default();
        self.set_active_cue_index(Some(current_index + 1), show);
    }
}

/// Determines how an [Executor] is activated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ButtonMode {
    #[default]
    Go,
}

impl FromStr for ButtonMode {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "go" => Ok(Self::Go),
            other => eyre::bail!("invalid button mode: '{other}'"),
        }
    }
}

/// Determines how an [Executor] is terminated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FaderMode {
    #[default]
    Master,
}

impl FromStr for FaderMode {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "never" => Ok(Self::Master),
            other => eyre::bail!("invalid fader mode: '{other}'"),
        }
    }
}
