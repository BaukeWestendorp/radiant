use gpui::{actions, impl_actions, AppContext, Global, Model};

use crate::{cmd::parser::CommandParser, show::Show};

use self::parser::ast;

mod parser;

impl_actions!(cmd, [Command]);
actions!(cmd, [RemoveCommand, ExecuteCommandList]);

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
pub enum Command {
    Group,
    Id(usize),
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
    pub fn global(cx: &AppContext) -> &Self {
        cx.global::<Self>()
    }

    pub fn commands(cx: &AppContext) -> &[Command] {
        &Self::global(cx).commands
    }

    pub fn push(command: Command, cx: &mut AppContext) {
        cx.update_global::<Self, _>(|command_list, _cx| {
            log::info!("{}", command);
            command_list.commands.push(command);
        })
    }

    pub fn extend(commands: impl IntoIterator<Item = Command>, cx: &mut AppContext) {
        cx.update_global::<Self, _>(|command_list, _cx| {
            for command in commands {
                log::info!("{}", command);
                command_list.commands.push(command);
            }
        })
    }

    pub fn remove_last(cx: &mut AppContext) {
        cx.update_global::<Self, _>(|command_list, _cx| {
            command_list.commands.pop();
        })
    }

    pub fn clear(cx: &mut AppContext) {
        cx.update_global::<Self, _>(|command_list, _cx| {
            command_list.commands.clear();
        })
    }

    pub fn handle_digit_key(c: char, cx: &mut AppContext) {
        match c {
            '0'..='9' => {
                let digit = c.to_digit(10).unwrap() as usize;
                Self::push(Command::Id(digit), cx);
            }
            _ => {}
        }
    }

    pub fn execute(show: Model<Show>, cx: &mut AppContext) {
        let commands = Self::commands(cx).iter().cloned();
        let Ok(action) = CommandParser::new(commands).parse_action() else {
            log::error!("Failed to parse command list");
            CommandList::clear(cx);
            return;
        };
        execute_action(action, show, cx);
        CommandList::clear(cx);
    }
}

impl Global for CommandList {}

pub(super) fn execute_action(
    action: ast::Action,
    show: gpui::Model<Show>,
    cx: &mut gpui::AppContext,
) {
    match action {
        ast::Action::SelectDataPoolItem(data_pool_item) => {
            show.update(cx, |show, cx| {
                log::info!("Selecting data pool item {:?}", data_pool_item);
                // FIXME: Implement
            });
        }
        ast::Action::NoOp => {}
    }
}
