use gpui::{actions, impl_actions};

use crate::command::parser::CommandParser;

mod parser;

pub use parser::{CommandAction, DataPoolItem};

impl_actions!(cmd, [Command]);
actions!(cmd, [RemoveCommand, ExecuteCommandList]);

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
pub enum Command {
    Clear,
    Group,
    Id(usize),
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clear => write!(f, "Clear"),
            Self::Group => write!(f, "Group"),
            Self::Id(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CommandList {
    commands: Vec<Command>,
}

impl CommandList {
    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn push(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn extend(&mut self, commands: impl IntoIterator<Item = Command>) {
        for command in commands {
            log::info!("{}", command);
            self.commands.push(command);
        }
    }

    pub fn last(&self) -> Option<&Command> {
        self.commands().last()
    }

    pub fn last_mut(&mut self) -> Option<&mut Command> {
        self.commands.last_mut()
    }

    pub fn remove_last(&mut self) {
        self.commands.pop();
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn handle_digit_key(&mut self, c: char) {
        match c {
            '0'..='9' => {
                let digit = c.to_digit(10).unwrap() as usize;
                match self.last_mut() {
                    Some(Command::Id(id)) => {
                        *id = *id * 10 + digit;
                    }
                    _ => {
                        self.push(Command::Id(digit));
                    }
                }
            }
            _ => {}
        }
    }

    pub fn is_complete(&self) -> bool {
        if self.commands() == &[Command::Clear] {
            return true;
        }

        return false;
    }

    pub fn parse(&mut self) -> Option<CommandAction> {
        let commands = self.commands().iter().cloned();
        let Ok(action) = CommandParser::new(commands).parse_action() else {
            log::error!("Failed to parse command list");
            self.clear();
            return None;
        };
        self.clear();
        Some(action)
    }
}
