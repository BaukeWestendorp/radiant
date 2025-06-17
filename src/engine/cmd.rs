use std::fmt::Display;

use crate::dmx;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    SetDmxValue(dmx::Address, dmx::Value),
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SetDmxValue(address, value) => write!(f, "set_dmx_value {address} {value}"),
        }
    }
}
