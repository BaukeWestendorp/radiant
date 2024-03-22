use std::fmt::Display;

use gpui::{actions, impl_actions};

impl_actions!(cmd, [Command]);
actions!(cmd, [RemoveCommand, ExecuteCommandList]);

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Command {
    instructions: Vec<Instruction>,
}

impl Command {
    pub fn parse(s: &str) -> Option<Self> {
        todo!();
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, instruction) in self.instructions.iter().enumerate() {
            let end = if i == self.instructions.len() - 1 {
                ""
            } else {
                " "
            };
            write!(f, "{}{}", instruction, end)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize)]
pub enum Instruction {
    Clear,
    Group(usize),
}

impl Instruction {
    pub fn parse(s: &str) -> Option<Self> {
        todo!();
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Instruction::Clear => write!(f, "Clear"),
            Instruction::Group(id) => write!(f, "Group {}", id),
        }
    }
}

mod test {

    #[test]
    fn parse_string_to_command_list() {
        use crate::command::{Command, Instruction};

        let str = "group 1";

        let command = Command::parse(str).unwrap();
        assert_eq!(
            command,
            Command {
                instructions: vec![Instruction::Group(1)]
            }
        );
    }
}
