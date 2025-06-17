use std::time::Duration;

use crate::backend::show::Show;
use crate::dmx;
use crate::engine::cmd::Command;
use crate::error::Result;

use crate::engine::Engine;

/// Starts the app in headless mode.
pub fn run(show: Show) -> Result<()> {
    let mut engine = Engine::new(show);
    engine.run()?;

    std::thread::sleep(Duration::from_secs_f32(1.0));
    engine.execute_command(Command::SetDmxValue(dmx::Address::default(), dmx::Value(42)));

    Ok(())
}
