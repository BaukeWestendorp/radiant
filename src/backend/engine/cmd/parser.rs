use std::iter::Peekable;

use crate::backend::object::{AnyObjectId, CueId, ExecutorId, SequenceId};

use super::lexer::{Lexer, Token};
use super::{Command, error::Error};

const CREATE: &str = "create";

const EXECUTOR: &str = "executor";
const SEQUENCE: &str = "sequence";
const CUE: &str = "cue";

pub struct Parser<'src> {
    lexer: Peekable<Lexer<'src>>,
}

impl<'src> Parser<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { lexer: Lexer::new(source).peekable() }
    }

    pub fn parse(mut self) -> Result<Command, Error> {
        let command = if self.expect_ident(CREATE) {
            let id = self.parse_object_id()?;
            let name = self.parse_string().ok().map(String::from);
            Command::Create { id, name }
        } else {
            match self.lexer.peek() {
                Some(token) => return Err(Error::UnexpectedToken(token.to_string())),
                None => return Err(Error::UnexpectedEOL),
            }
        };

        if self.lexer.next().is_some() {
            return Err(Error::ExpectedEOL);
        }

        Ok(command)
    }

    fn parse_ident(&mut self) -> Result<&str, Error> {
        match self.next_token()? {
            Token::Ident(ident) => Ok(ident),
            _ => Err(Error::ExpectedIdent),
        }
    }

    fn parse_string(&mut self) -> Result<&str, Error> {
        match self.next_token()? {
            Token::String(string) => Ok(string),
            _ => Err(Error::ExpectedString),
        }
    }

    fn parse_int(&mut self) -> Result<i64, Error> {
        match self.next_token()? {
            Token::Integer(int) => Ok(int),
            _ => Err(Error::ExpectedInteger),
        }
    }

    fn parse_object_id(&mut self) -> Result<AnyObjectId, Error> {
        let ident = self.parse_ident()?.to_string();
        let n = self.parse_int()?;

        if n < 0 {
            return Err(Error::NegativeObjectId);
        }

        match ident.as_str() {
            EXECUTOR => Ok(ExecutorId(n as u32).into()),
            SEQUENCE => Ok(SequenceId(n as u32).into()),
            CUE => Ok(CueId(n as u32).into()),
            _ => Err(Error::ExpectedObjectId),
        }
    }

    fn expect_ident(&mut self, name: &'src str) -> bool {
        self.next_token().is_ok_and(|t| matches!(t, Token::Ident(s) if s == name))
    }

    fn next_token(&mut self) -> Result<Token, Error> {
        self.lexer.next().ok_or(Error::UnexpectedEOL)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        backend::{engine::cmd::Command, object::AnyObjectId},
        cmd,
    };

    #[test]
    fn test_create_executor_with_name() {
        assert_eq!(
            cmd!("create executor 0 \"Example Executor\""),
            Command::Create {
                id: AnyObjectId::Executor(0.into()),
                name: Some("Example Executor".to_string()),
            },
        );
    }

    #[test]
    fn test_create_cue_with_name() {
        assert_eq!(
            cmd!("create cue 0 \"Example Cue\""),
            Command::Create {
                id: AnyObjectId::Cue(0.into()),
                name: Some("Example Cue".to_string()),
            },
        );
    }

    #[test]
    fn test_create_sequence_with_name() {
        assert_eq!(
            cmd!("create sequence 0 \"Example Sequence\""),
            Command::Create {
                id: AnyObjectId::Sequence(0.into()),
                name: Some("Example Sequence".to_string()),
            },
        );
    }

    // insert cue 0 recipe [ (fixture_group 0, preset::dimmer 0) ]
    // insert sequence 0 cue 0
    // update sequence 0 active_cue 0
}
