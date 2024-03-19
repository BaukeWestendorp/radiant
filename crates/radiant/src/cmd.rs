use std::iter::Peekable;

use gpui::{actions, impl_actions, AppContext, Global, Model};

use crate::show::Show;

impl_actions!(cmd, [Command]);
actions!(cmd, [RemoveCommand, ExecuteCommandList]);

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
pub enum Command {
    Group,
    Id(usize),
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
            log::info!("{:?}", command);
            command_list.commands.push(command);
        })
    }

    pub fn extend(commands: impl IntoIterator<Item = Command>, cx: &mut AppContext) {
        cx.update_global::<Self, _>(|command_list, _cx| {
            for command in commands {
                log::info!("{:?}", command);
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
        let action = CommandParser::new(commands).parse_action();
        CommandList::clear(cx);
        dbg!(&action);
    }
}

impl Global for CommandList {}

struct CommandParser<I>
where
    I: Iterator<Item = Command>,
{
    commands: Peekable<I>,
}

impl<I> CommandParser<I>
where
    I: Iterator<Item = Command>,
{
    pub fn new(commands: I) -> Self {
        Self {
            commands: commands.into_iter().peekable(),
        }
    }

    pub fn parse_action(&mut self) -> ParserResult<ast::Action> {
        let action = if let Some(data_pool_item) = self.parse_data_pool_item()? {
            Ok(ast::Action::SelectDataPoolItem(data_pool_item))
        } else {
            Err(ParserError)
        };

        // Ensure that we have consumed all the input.
        if self.peek().is_ok() {
            Err(ParserError)
        } else {
            action
        }
    }

    fn parse_data_pool_item(&mut self) -> ParserResult<Option<ast::DataPoolItem>> {
        match self.peek()? {
            Command::Group => {
                self.consume(&Command::Group)?;
                let id = self.parse_data_pool_id()?;
                Ok(Some(ast::DataPoolItem::Group(id)))
            }
            _ => Ok(None),
        }
    }

    fn parse_data_pool_id(&mut self) -> ParserResult<usize> {
        match self.next()? {
            Command::Id(id) => Ok(id),
            _ => Err(ParserError),
        }
    }

    /// Look-ahead one command and see what kind of command it is.
    fn peek(&mut self) -> ParserResult<&Command> {
        self.commands.peek().map(Ok).unwrap_or(Err(ParserError))
    }

    /// Get the next command.
    fn next(&mut self) -> ParserResult<Command> {
        self.commands.next().map(Ok).unwrap_or(Err(ParserError))
    }

    /// Move forward one command in the input and check
    /// that we pass the kind of command we expect.
    fn consume(&mut self, expected: &Command) -> ParserResult<Command> {
        let next = self.next()?;
        if next == *expected {
            Ok(next)
        } else {
            Err(ParserError)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ParserError;

pub type ParserResult<V> = Result<V, ParserError>;

mod ast {
    #[derive(Debug, Clone, Copy)]
    pub(super) enum Action {
        SelectDataPoolItem(DataPoolItem),
        NoOp,
    }

    #[derive(Debug, Clone, Copy)]
    pub(super) enum DataPoolItem {
        Group(usize),
    }
}
