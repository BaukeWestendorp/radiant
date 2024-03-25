use std::fmt::Display;

use itertools::Itertools;

use self::parser::{Parser, ParserResult};

mod lexer;
mod parser;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Clear,
    Select(Object),
    Go,
    Top,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Instruction::Clear => write!(f, "Clear"),
            Instruction::Select(object) => write!(f, "Group {object}"),
            Instruction::Go => write!(f, "Go"),
            Instruction::Top => write!(f, "Go"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Object {
    Fixture(usize),
    Group(usize),
    Executor(usize),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Fixture(id) => write!(f, "Fixture {id}"),
            Object::Group(id) => write!(f, "Group {id}"),
            Object::Executor(id) => write!(f, "Executor {id}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub instructions: Vec<Instruction>,
}

impl Command {
    pub fn new(instructions: impl IntoIterator<Item = Instruction>) -> Self {
        Self {
            instructions: instructions.into_iter().collect_vec(),
        }
    }

    pub fn parse(s: &str) -> ParserResult<Self> {
        let mut parser = Parser::new(s);
        Ok(Self {
            instructions: parser.parse_instructions()?,
        })
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[cfg(test)]
mod tests {
    use crate::command::Object;

    macro_rules! instructions {
        ($input:expr, $instructions:expr) => {{
            use crate::command::{Command, Instruction};
            let command = Command::parse($input).unwrap();
            assert_eq!(
                command,
                Command {
                    instructions: $instructions
                }
            );
        }};
    }

    macro_rules! error {
        ($input:expr, $error:expr) => {{
            use crate::command::Command;
            let result = Command::parse($input);
            assert_eq!(result, Err($error));
        }};
    }

    #[test]
    fn parse_string_to_instruction() {
        instructions!(
            "Select Group 1",
            vec![Instruction::Select(Object::Group(1))]
        );
        instructions!(
            "Select Group 42",
            vec![Instruction::Select(Object::Group(42))]
        );
        instructions!(
            "Select Fixture 1",
            vec![Instruction::Select(Object::Fixture(1))]
        );

        instructions!("Clear", vec![Instruction::Clear]);
    }

    #[test]
    fn parse_string_error() {
        use crate::command::lexer::token::{Token, TokenKind};
        use crate::command::parser::{ParserError, ParserErrorKind};
        use crate::command::Span;

        error!(
            "Select Group",
            ParserError {
                kind: ParserErrorKind::ExpectedId,
                token: Some(Token {
                    kind: TokenKind::EndOfLine,
                    span: Span { start: 12, end: 12 },
                })
            }
        );

        error!(
            "Select Group 1a",
            ParserError {
                kind: ParserErrorKind::UnexpectedToken,
                token: Some(Token {
                    kind: TokenKind::Invalid,
                    span: Span { start: 14, end: 15 },
                })
            }
        );
    }
}
