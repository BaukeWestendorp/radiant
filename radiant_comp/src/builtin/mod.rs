use crate::engine::Engine;
use crate::error::Result;

mod comp;

pub use comp::*;

pub(crate) fn register(engine: &mut Engine) -> Result<()> {
    comp::objects::register(engine)?;
    comp::patch::register(engine)?;
    comp::pools::register(engine)?;
    comp::programmer::register(engine)?;
    Ok(())
}
