use std::str::FromStr;

use crate::{Cue, Sequence, SequenceId, Show};

crate::define_object_id!(ExecutorId);

/// An executor controls how a sequence will be activated and terminated.
#[derive(Debug, Clone, PartialEq)]
pub struct Executor {
    id: ExecutorId,
    pub name: String,
    pub(crate) button: ExecutorButton,
    pub(crate) fader: ExecutorFader,
    pub(crate) sequence_id: Option<SequenceId>,
    pub(crate) master_level: f32,
    active_cue_index: Option<usize>,
}

impl Executor {
    pub fn new(id: impl Into<ExecutorId>) -> Self {
        Self {
            id: id.into(),
            name: "New Executor".to_string(),
            button: ExecutorButton::default(),
            fader: ExecutorFader::default(),
            sequence_id: None,
            master_level: 1.0,
            active_cue_index: None,
        }
    }

    pub fn id(&self) -> ExecutorId {
        self.id
    }

    pub fn button(&self) -> &ExecutorButton {
        &self.button
    }

    pub fn fader(&self) -> &ExecutorFader {
        &self.fader
    }

    pub fn sequence_id(&self) -> Option<&SequenceId> {
        self.sequence_id.as_ref()
    }

    /// Gets a reference to the [Sequence] this executor is linked to.
    pub fn sequence<'a>(&self, show: &'a Show) -> Option<&'a Sequence> {
        let sequence_id = self.sequence_id?;
        show.sequence(sequence_id).or_else(move || {
            log::warn!("sequence with id {} not found", sequence_id);
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

        // FIXME: We should add an option to change the end-of-sequence behaviour.
        //        For now we just loop...
        if index > sequence.len() - 1 {
            self.active_cue_index = Some(0);
        } else {
            self.active_cue_index = Some(index);
        }
    }

    pub fn active_cue_index(&self) -> Option<usize> {
        self.active_cue_index
    }

    /// Gets a reference to the [Cue] that is currently activated by the executor.
    pub fn active_cue<'a>(&self, show: &'a Show) -> Option<&'a Cue> {
        let index = self.active_cue_index?;
        let cue_id = self.sequence(show)?.cues().get(index)?;
        show.cue(*cue_id)
    }

    pub fn master_level(&self) -> f32 {
        self.master_level
    }

    pub fn manage_state(&mut self, show: &Show) {
        match self.button.mode {
            ExecutorButtonMode::Go => {
                if self.button.was_pressed() {
                    self.go(show)
                }
            }
        }

        match self.fader.mode {
            ExecutorFaderMode::Master => self.master_level = self.fader.level(),
        }

        self.button.reset_state();
    }

    pub fn go(&mut self, show: &Show) {
        let new_index = self.active_cue_index.map(|ix| ix + 1).unwrap_or_default();
        self.set_active_cue_index(Some(new_index), show);
    }
}

impl FromStr for ExecutorButtonMode {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "go" => Ok(Self::Go),
            other => eyre::bail!("invalid button mode: '{other}'"),
        }
    }
}

impl FromStr for ExecutorFaderMode {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "master" => Ok(Self::Master),
            other => eyre::bail!("invalid fader mode: '{other}'"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExecutorButton {
    mode: ExecutorButtonMode,
    was_pressed: bool,
    currently_pressed: bool,
}

impl ExecutorButton {
    pub fn mode(&self) -> ExecutorButtonMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: ExecutorButtonMode) {
        self.mode = mode;
    }

    pub fn press(&mut self) {
        self.was_pressed = true;
        self.currently_pressed = true;
    }

    pub fn release(&mut self) {
        self.currently_pressed = false;
    }

    pub fn currently_pressed(&self) -> bool {
        self.currently_pressed
    }

    pub fn was_pressed(&self) -> bool {
        self.was_pressed
    }

    fn reset_state(&mut self) {
        self.was_pressed = false;
    }
}

/// Determines the function of an [Executor]s button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ExecutorButtonMode {
    #[default]
    Go,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExecutorFader {
    mode: ExecutorFaderMode,
    level: f32,
}

impl ExecutorFader {
    pub fn mode(&self) -> ExecutorFaderMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: ExecutorFaderMode) {
        self.mode = mode;
    }

    pub fn set_level(&mut self, level: f32) {
        self.level = level.clamp(0.0, 1.0);
    }

    pub fn level(&self) -> f32 {
        self.level
    }
}

/// Determines the function of an [Executor]s fader.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ExecutorFaderMode {
    #[default]
    Master,
}
