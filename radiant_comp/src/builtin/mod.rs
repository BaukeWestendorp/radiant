use crate::engine::Engine;
use crate::error::Result;

mod cmd;
mod comp;

pub use cmd::*;
pub use comp::*;

pub(crate) fn register(engine: &mut Engine) -> Result<()> {
    cmd::register(engine);
    comp::objects::register(engine)?;
    comp::patch::register(engine)?;
    comp::pools::register(engine)?;
    comp::programmer::register(engine)?;
    Ok(())
}
