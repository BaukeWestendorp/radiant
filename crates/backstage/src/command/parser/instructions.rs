use crate::command::lexer::token::{Token, TokenKind};
use crate::command::{Instruction, Object};

use super::{Parser, ParserError, ParserErrorKind, ParserResult};

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub fn parse_instructions(&mut self) -> ParserResult<Vec<Instruction>> {
        let mut instructions = vec![];
        while self.peek() != TokenKind::EndOfLine {
            let instruction = self.parse_instruction()?;
            instructions.push(instruction)
        }
        Ok(instructions)
    }

    pub fn parse_instruction(&mut self) -> ParserResult<Instruction> {
        match self.peek() {
            TokenKind::Clear => {
                self.consume(TokenKind::Clear)?;
                Ok(Instruction::Clear)
            }
            TokenKind::Select => {
                self.consume(TokenKind::Select)?;
                let object = self.parse_object()?;
                Ok(Instruction::Select(object))
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

    pub fn parse_object(&mut self) -> ParserResult<Object> {
        let kind = self.peek();
        let token = self.consume(kind)?;

        let id = self.parse_id()?;

        let instruction = match kind {
            TokenKind::Group => Object::Group(id),
            TokenKind::Fixture => Object::Fixture(id),
            _ => {
                return Err(ParserError {
                    kind: ParserErrorKind::ExpectedObject,
                    token: Some(token),
                })
            }
        };

        Ok(instruction)
    }

    pub fn parse_id(&mut self) -> ParserResult<usize> {
        let id_token = self.next()?;
        let id_str = self.text(&id_token).to_string();
        id_str.parse().map_err(|_| ParserError {
            kind: ParserErrorKind::ExpectedId,
            token: Some(id_token),
        })
    }
}
