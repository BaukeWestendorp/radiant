use std::fmt::Display;

use self::parser::instructions::Instruction;
use self::parser::{Parser, ParserResult};

mod lexer;
mod parser;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Command {
    instructions: Vec<Instruction>,
}

impl Command {
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

#[cfg(test)]
mod tests {
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
        instructions!("Group 1", vec![Instruction::Group(1)]);
        instructions!("Group 42", vec![Instruction::Group(42)]);

        instructions!("Clear", vec![Instruction::Clear]);
    }

    #[test]
    fn parse_string_error() {
        use crate::command::lexer::token::{Token, TokenKind};
        use crate::command::parser::{ParserError, ParserErrorKind};
        use crate::command::Span;

        error!(
            "Group",
            ParserError {
                kind: ParserErrorKind::ExpectedId,
                token: Some(Token {
                    kind: TokenKind::EndOfLine,
                    span: Span { start: 5, end: 5 },
                })
            }
        );

        error!(
            "Group 1a",
            ParserError {
                kind: ParserErrorKind::UnexpectedToken,
                token: Some(Token {
                    kind: TokenKind::Invalid,
                    span: Span { start: 7, end: 8 },
                })
            }
        );
    }
}
