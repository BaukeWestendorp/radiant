use self::lexer::Token;
use crate::{
    cmd::lexer::Lexer,
    show::{FixtureId, Show},
};

mod lexer;

// FIXME: We should deserialize this from a string by parsing.
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize)]
pub enum Object {
    Fixture(Option<FixtureId>),
    Group(Option<usize>),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Fixture(id) => write!(
                f,
                "fixture {}",
                id.map(|id| id.to_string()).unwrap_or_default()
            ),
            Object::Group(id) => write!(
                f,
                "group {}",
                id.map(|id| id.to_string()).unwrap_or_default()
            ),
        }
    }
}

// FIXME: We should deserialize this from a string by parsing.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub enum Command {
    Clear,
    Select(Option<Object>),
}

impl Command {
    pub fn parse(input: impl AsRef<str>) -> Result<Command, Error> {
        let mut lexer = Lexer::new(input.as_ref());

        macro_rules! confirm_end_of_command {
            ($token:expr) => {
                if lexer.next_token()?.is_some() {
                    return Err(Error::UnexpectedToken($token));
                }
            };
        }

        let command = match lexer.next_token()? {
            Some((token, _start, _end)) => match token {
                Token::Clear => {
                    confirm_end_of_command!(token);
                    Command::Clear
                }
                Token::Select => {
                    let object = parse_object(&mut lexer)?;
                    confirm_end_of_command!(token);
                    Command::Select(Some(object))
                }
                other => return Err(Error::UnexpectedToken(other)),
            },
            None => return Err(Error::UnexpectedEndOfInput),
        };
        Ok(command)
    }
}

fn parse_object(lexer: &mut Lexer) -> Result<Object, Error> {
    let object = match lexer.next_token()? {
        Some((Token::Fixture, _start, _end)) => {
            let (number_token, _start, _end) =
                lexer.next_token()?.ok_or_else(|| Error::ExpectedNumber)?;
            match number_token {
                Token::Number(number) => Object::Fixture(Some(FixtureId::new(number))),
                _ => return Err(Error::ExpectedNumber),
            }
        }
        Some((Token::Group, _start, _end)) => {
            let (number_token, _start, _end) =
                lexer.next_token()?.ok_or_else(|| Error::ExpectedNumber)?;
            match number_token {
                Token::Number(number) => Object::Group(Some(number)),
                _ => return Err(Error::ExpectedNumber),
            }
        }
        Some((other, _, _)) => return Err(Error::UnexpectedToken(other)),
        None => return Err(Error::UnexpectedEndOfInput),
    };
    Ok(object)
}

fn parse_string(lexer: &mut Lexer) -> Result<String, Error> {
    let (string_token, _start, _end) = lexer.next_token()?.ok_or_else(|| Error::ExpectedString)?;
    match string_token {
        Token::String(string) => Ok(string),
        _ => Err(Error::ExpectedString),
    }
}

fn consume(lexer: &mut Lexer, expected: Token) -> Result<(), Error> {
    let (token, _start, _end) = lexer
        .next_token()?
        .ok_or_else(|| Error::ExpectedToken(expected.clone()))?;
    if token != expected {
        return Err(Error::ExpectedToken(expected));
    }
    Ok(())
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Command::Clear => write!(f, "clear"),
            Command::Select(Some(object)) => write!(f, "select {}", object),
            Command::Select(None) => write!(f, "select"),
        }
    }
}

impl Show {
    pub fn execute_command(&mut self, command: &Command) -> Result<(), Error> {
        match command {
            Command::Clear => {
                if self.selected_fixture_ids().is_empty() {
                    self.programmer_mut().clear_changes()
                } else {
                    self.clear_selection();
                }
            }
            Command::Select(object) => match object {
                Some(Object::Fixture(id)) => {
                    let Some(id) = id else {
                        return Err(Error::ExecutionError("No fixture id provided".to_string()));
                    };

                    if !self.patchlist().fixture(id).is_some() {
                        return Err(Error::ExecutionError(format!(
                            "Fixture with id '{id}' not found"
                        )));
                    }

                    if !self.selected_fixture_ids().contains(id) {
                        self.select_fixture(*id);
                    } else {
                        log::debug!("Fixture with id '{id}' already selected");
                    }
                }
                Some(Object::Group(id)) => {
                    let Some(id) = id else {
                        return Err(Error::ExecutionError("No group id provided".to_string()));
                    };

                    let group = self
                        .data()
                        .group(*id)
                        .ok_or_else(|| {
                            Error::ExecutionError(format!("Group with id '{id}' not found"))
                        })?
                        .clone();
                    for fixture_id in group.fixtures.iter() {
                        self.execute_command(&Command::Select(Some(Object::Fixture(Some(
                            *fixture_id,
                        )))))?;
                    }
                }
                None => {
                    return Err(Error::ExecutionError(
                        "Select requires a target object".to_string(),
                    ))
                }
            },
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unexpected token: {0}")]
    UnexpectedToken(Token),
    #[error("Expected number")]
    ExpectedNumber,
    #[error("Expected string")]
    ExpectedString,
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Expected token: {0}")]
    ExpectedToken(Token),
    #[error("Execution error: {0}")]
    ExecutionError(String),
    #[error("Tokenization error: {0}")]
    TokenizationError(String),
}

#[cfg(test)]
mod tests {
    use crate::cmd::{Command, Object};

    #[test]
    fn test_parse_case_insensitivity() {
        let expected = Command::Clear;
        assert_eq!(Command::parse("clear").unwrap(), expected);
        assert_eq!(Command::parse("Clear").unwrap(), expected);
        assert_eq!(Command::parse("CLEAR").unwrap(), expected);

        let expected = Command::Select(Some(Object::Group(Some(1))));
        assert_eq!(Command::parse(r#"select GrouP 1"#).unwrap(), expected);
    }

    #[test]
    fn test_whitespace() {
        let expected = Command::Select(Some(Object::Group(Some(42))));
        assert_eq!(Command::parse("select group 42").unwrap(), expected);
        assert_eq!(Command::parse("select   group 42").unwrap(), expected);
        assert_eq!(Command::parse("select     group     42").unwrap(), expected);
    }

    #[test]
    fn test_parse_clear() {
        let expected = Command::Clear;

        assert_eq!(Command::parse("clear").unwrap(), expected);
        assert!(Command::parse("clear foobar").is_err());
    }

    #[test]
    fn test_parse_select() {
        let expected = Command::Select(Some(Object::Group(Some(42))));

        assert_eq!(Command::parse("select group 42").unwrap(), expected);
        assert!(Command::parse("select group 42 foobar").is_err());
        assert!(Command::parse("select foobar group 42").is_err());
        assert!(Command::parse("select group foobar 42").is_err());
    }
}
