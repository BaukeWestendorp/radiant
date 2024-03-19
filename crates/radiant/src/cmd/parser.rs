use std::iter::Peekable;

use super::Command;

pub mod ast {
    #[derive(Debug, Clone, Copy)]
    pub enum Action {
        SelectDataPoolItem(DataPoolItem),
    }

    #[derive(Debug, Clone, Copy)]
    pub enum DataPoolItem {
        Group(usize),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ParserError;

pub type ParserResult<V> = Result<V, ParserError>;

pub struct CommandParser<I>
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
