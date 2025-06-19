use std::str::FromStr;

use crate::backend::object::AnyObjectId;
use crate::backend::patch::attr::Attribute;
use crate::backend::patch::attr::AttributeValue;
use crate::backend::patch::fixture::DmxMode;
use crate::backend::patch::fixture::FixtureId;
use crate::dmx;

pub mod error;
mod lexer;
mod parser;

#[macro_export]
macro_rules! cmd {
    ($cmd_str:expr) => {{
        use std::str::FromStr;
        Command::from_str($cmd_str).unwrap()
    }};
}

/// A [Command] is the interface between the engine and the backend.
#[derive(Debug, Clone, PartialEq)]
#[derive(facet::Facet)]
#[repr(u8)]
pub enum Command {
    Patch(PatchCommand),
    Programmer(ProgrammerCommand),
    Create { id: AnyObjectId, name: Option<String> },
    // TODO: get    <obj_type> <id> [data/field]
    // TODO: update <obj_type> <id> [data/field] <new_value>
    // TODO: remove <obj_type> <id>
    // TODO: rename <obj_type> <id> <new_name>
    // TODO: select <obj_type> <id>
}

impl FromStr for Command {
    type Err = error::Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        parser::Parser::new(source).parse()
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(facet::Facet)]
#[repr(u8)]
pub enum PatchCommand {
    Add { fixture_id: FixtureId, address: dmx::Address, gdtf_file_name: String, mode: DmxMode },
    // TODO: patch update <fixture_id> [address|mode|gdtf_file_name] <new_value>
    // TODO: patch remove <fixture_id>
}

#[derive(Debug, Clone, PartialEq)]
#[derive(facet::Facet)]
#[repr(u8)]
pub enum ProgrammerCommand {
    Set(ProgrammerSetCommand),
}

#[derive(Debug, Clone, PartialEq)]
#[derive(facet::Facet)]
#[repr(u8)]
pub enum ProgrammerSetCommand {
    Direct { address: dmx::Address, value: dmx::Value },
    Attribute { fixture_id: FixtureId, attribute: Attribute, value: AttributeValue },
}
