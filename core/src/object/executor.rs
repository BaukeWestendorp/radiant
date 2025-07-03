use std::str::FromStr;

use crate::object::{Cue, Sequence, SequenceId};
use crate::show::Show;

super::define_object_id!(ExecutorId);

/// An Executor is responsible for controlling playback of sequences through
/// button presses and fader movements.
///
/// Executors contain a button and a fader, each with configurable modes to
/// determine their behavior. They can be assigned a sequence which contains
/// cues to be played back.
#[derive(Debug, Clone, PartialEq)]
pub struct Executor {
    id: ExecutorId,
    pub(crate) name: String,
    pub(crate) button: ExecutorButton,
    pub(crate) fader: ExecutorFader,
    pub(crate) sequence_id: Option<SequenceId>,
    pub(crate) master_level: f32,
    active_cue_index: Option<usize>,
}

impl Executor {
    /// Creates a new [Executor] with the specified id.
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

    /// Returns this executor's id.
    pub fn id(&self) -> ExecutorId {
        self.id
    }

    /// Returns the name of this executor.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets this executor's button.
    pub fn button(&self) -> &ExecutorButton {
        &self.button
    }

    /// Gets this executor's fader.
    pub fn fader(&self) -> &ExecutorFader {
        &self.fader
    }

    /// Gets the associated [SequenceId] if it has one.
    pub fn sequence_id(&self) -> Option<&SequenceId> {
        self.sequence_id.as_ref()
    }

    /// Gets a reference to this executor's associated [Sequence].
    ///
    /// Returns `None` if no sequence is assigned or if the sequence cannot be
    /// found in the show.
    pub fn sequence<'a>(&self, show: &'a Show) -> Option<&'a Sequence> {
        let sequence_id = self.sequence_id?;
        show.sequence(sequence_id).or_else(move || {
            log::warn!("sequence with id {} not found", sequence_id);
            None
        })
    }

    /// Sets the index indicating the active [Cue].
    ///
    /// For now, if the index exceeds the sequence length, it wraps to the
    /// beginning. Setting `None` clears the active cue.
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

    /// Gets the index of the currently activated cue in this executor's
    /// associated [Sequence].
    ///
    /// Returns `None` if no cue is currently active.
    pub fn active_cue_index(&self) -> Option<usize> {
        self.active_cue_index
    }

    /// Gets a reference to the [Cue] that is currently activated by the
    /// executor.
    ///
    /// Returns `None` if no cue is active, no sequence is assigned, or if the
    /// active cue cannot be found in the show.
    pub fn active_cue<'a>(&self, show: &'a Show) -> Option<&'a Cue> {
        let index = self.active_cue_index?;
        let cue_id = self.sequence(show)?.cues().get(index)?;
        show.cue(*cue_id)
    }

    /// Gets the current master level of the executor.
    ///
    /// This value can be controlled by the fader when in Master mode.
    pub fn master_level(&self) -> f32 {
        self.master_level
    }

    /// Manages the internal state of the executor based on button and fader
    /// interactions.
    ///
    /// This method should be called regularly to process button presses and
    /// fader movements according to their configured modes.
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

    /// Advances to the next cue in the sequence.
    ///
    /// If no cue is currently active, starts from the first cue.
    /// Wraps to the beginning when reaching the end of the sequence.
    pub fn go(&mut self, show: &Show) {
        let new_index = self.active_cue_index.map(|ix| ix + 1).unwrap_or_default();
        self.set_active_cue_index(Some(new_index), show);
    }
}

/// Represents the button component of an executor.
///
/// The button can be configured with different modes to control its behavior
/// and tracks both current and previous press states.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExecutorButton {
    mode: ExecutorButtonMode,
    was_pressed: bool,
    currently_pressed: bool,
}

impl ExecutorButton {
    /// Gets the current mode of the button.
    pub fn mode(&self) -> ExecutorButtonMode {
        self.mode
    }

    /// Sets the mode of the button, determining its behavior.
    pub fn set_mode(&mut self, mode: ExecutorButtonMode) {
        self.mode = mode;
    }

    /// Simulates pressing the button.
    ///
    /// Sets both the current pressed state and the "was pressed" flag.
    pub fn press(&mut self) {
        self.was_pressed = true;
        self.currently_pressed = true;
    }

    /// Simulates releasing the button.
    ///
    /// Clears the current pressed state but maintains the "was pressed" flag
    /// until the next state reset.
    pub fn release(&mut self) {
        self.currently_pressed = false;
    }

    /// Returns whether the button is currently being pressed.
    pub fn currently_pressed(&self) -> bool {
        self.currently_pressed
    }

    /// Returns whether the button was pressed since the last state reset.
    ///
    /// This flag is used to detect button press events and is cleared
    /// after each call to `reset_state()`.
    pub fn was_pressed(&self) -> bool {
        self.was_pressed
    }

    fn reset_state(&mut self) {
        self.was_pressed = false;
    }
}

/// Determines the function of an [Executor]'s button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ExecutorButtonMode {
    /// Button acts as a "Go" button, advancing to the next cue when pressed.
    #[default]
    Go,
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

/// Represents the fader component of an executor.
///
/// The fader can be configured with different modes to control its behavior
/// and maintains a level value between 0.0 and 1.0.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExecutorFader {
    mode: ExecutorFaderMode,
    level: f32,
}

impl ExecutorFader {
    /// Gets the current mode of the fader.
    pub fn mode(&self) -> ExecutorFaderMode {
        self.mode
    }

    /// Sets the mode of the fader, determining its behavior.
    pub fn set_mode(&mut self, mode: ExecutorFaderMode) {
        self.mode = mode;
    }

    /// Sets the level of the fader.
    ///
    /// The level is automatically clamped to the range [0.0, 1.0].
    pub fn set_level(&mut self, level: f32) {
        self.level = level.clamp(0.0, 1.0);
    }

    /// Gets the current level of the fader.
    ///
    /// Returns a value between 0.0 and 1.0.
    pub fn level(&self) -> f32 {
        self.level
    }
}

/// Determines the function of an [Executor]'s fader.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ExecutorFaderMode {
    /// Fader controls the master level of the executor.
    #[default]
    Master,
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
