use crate::command::lexer::token::{Token, TokenKind};

use super::{Parser, ParserError, ParserErrorKind, ParserResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Clear,
    Group(usize),
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Instruction::Clear => write!(f, "Clear"),
            Instruction::Group(id) => write!(f, "Group {}", id),
        }
    }
}

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub fn parse_instruction(&mut self) -> ParserResult<Instruction> {
        match self.peek() {
            TokenKind::Clear => {
                self.consume(TokenKind::Clear)?;
                Ok(Instruction::Clear)
            }
            TokenKind::Group => {
                self.consume(TokenKind::Group)?;
                let id = {
                    let id_token = self.next().unwrap();
                    let id_str = self.text(&id_token).to_string();
                    id_str.parse().map_err(|_| ParserError {
                        kind: ParserErrorKind::ExpectedId,
                        token: Some(id_token),
                    })
                }?;

                Ok(Instruction::Group(id))
            }
            TokenKind::Invalid => {
                let invalid_token = self.consume(TokenKind::Invalid)?;

                Err(ParserError {
                    kind: ParserErrorKind::UnexpectedToken,
                    token: Some(invalid_token),
                })
            }
            _ => unreachable!(),
        }
    }
}
