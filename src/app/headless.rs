use eyre::Context;

use crate::backend::engine::Engine;
use crate::backend::engine::cmd::Command;
use crate::backend::object::{
    ActivationMode, Cue, CueContent, DimmerPresetId, Executor, FixtureGroupId, Object, Recipe,
    Sequence, TerminationMode,
};
use crate::error::Result;
use crate::showfile::Showfile;

/// Starts the app in headless mode.
pub fn run(showfile: Showfile) -> Result<()> {
    let mut engine = Engine::new(showfile).context("Failed to create engine")?;
    engine.start().context("Failed to start engine")?;

    let sequence = Sequence::new(0).with_name("Example Sequence").with_cue(
        Cue::new(CueContent::Recipe(
            Recipe::new().with_combination(FixtureGroupId(0), DimmerPresetId(0)),
        ))
        .with_name("Example Cue"),
    );

    engine.execute_command(Command::New(Object::Executor(
        Executor::new(0)
            .with_name("Example Executor")
            .with_activation_mode(ActivationMode::Instant)
            .with_termination_mode(TerminationMode::Never)
            .with_sequence(sequence.id),
    )))?;

    Ok(())
}
