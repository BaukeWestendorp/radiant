use gpui::{actions, impl_actions, AppContext, Global, Model};

use crate::{cmd::parser::CommandParser, show::Show};

use self::parser::ast::{Action, DataPoolItem};

mod parser;

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
    pub fn global(cx: &AppContext) -> &Self {
        cx.global::<Self>()
    }

    pub fn update_global<F, R>(cx: &mut AppContext, f: F) -> R
    where
        F: FnOnce(&mut Self, &mut AppContext) -> R,
    {
        cx.update_global(f)
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

    pub fn last(cx: &AppContext) -> Option<&Command> {
        Self::commands(cx).last()
    }

    pub fn remove_last(cx: &mut AppContext) {
        cx.update_global::<Self, _>(|command_list, _cx| {
            command_list.commands.pop();
        })
    }

    pub fn update_last<F>(f: F, cx: &mut AppContext)
    where
        F: FnOnce(&mut Command),
    {
        cx.update_global::<Self, _>(|command_list, _cx| {
            if let Some(command) = command_list.commands.last_mut() {
                f(command);
            }
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
                match Self::last(cx) {
                    Some(Command::Id(_)) => {
                        Self::update_last(
                            |command| {
                                if let Command::Id(id) = command {
                                    *id = *id * 10 + digit;
                                }
                            },
                            cx,
                        );
                    }
                    _ => {
                        Self::push(Command::Id(digit), cx);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn is_complete(cx: &AppContext) -> bool {
        if Self::commands(cx) == &[Command::Clear] {
            return true;
        }

        return false;
    }

    pub fn execute(show: Model<Show>, cx: &mut AppContext) {
        let commands = Self::commands(cx).iter().cloned();
        let Ok(action) = CommandParser::new(commands).parse_action() else {
            log::error!("Failed to parse command list");
            Self::clear(cx);
            return;
        };
        execute_action(action, show, cx);
        Self::clear(cx);
    }
}

impl Global for CommandList {}

pub(super) fn execute_action(action: Action, show: gpui::Model<Show>, cx: &mut gpui::AppContext) {
    match action {
        Action::SelectDataPoolItem(data_pool_item) => {
            show.update(cx, |show, _cx| match &data_pool_item {
                DataPoolItem::Group(id) => {
                    let Some(group) = show.data_pools.group(*id) else {
                        log::error!("Group {} not found", id);
                        return;
                    };

                    let ids = group.fixtures.clone();
                    show.programmer.select_fixtures(ids);
                }
            });
        }
        Action::ClearProgrammer => {
            show.update(cx, |show, _cx| {
                show.programmer.clear();
            });
        }
    }
}
