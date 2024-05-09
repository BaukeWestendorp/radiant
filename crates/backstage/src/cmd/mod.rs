//! # Commands
//!
//! This module contains the definition of commands.
//! Commands are parsed from strings and can be executed on a show to change its state.

use self::lexer::Token;
use crate::{
    cmd::lexer::Lexer,
    show::{FixtureId, PresetFilter, Show},
};

mod lexer;

// FIXME: We should deserialize this from a string by parsing.
/// A representation of an object in the show.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub enum Object {
    /// A fixture in the show.
    Fixture(Option<FixtureId>),
    /// A group in the show.
    Group(Option<usize>),
    /// A preset in the show.
    Preset(Option<PresetFilter>, Option<usize>),
    /// An effect in the show.
    Effect(Option<usize>),
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
            Object::Preset(filter, id) => write!(
                f,
                "preset {}::{}",
                filter
                    .as_ref()
                    .map(|filter| filter.to_string())
                    .unwrap_or_default(),
                id.map(|id| id.to_string()).unwrap_or_default()
            ),
            Object::Effect(id) => write!(
                f,
                "effect {}",
                id.map(|id| id.to_string()).unwrap_or_default()
            ),
        }
    }
}

// FIXME: We should deserialize this from a string by parsing.
/// A command that can be executed in the show.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub enum Command {
    /// If the show has some fixtures selected, it will clear these.
    /// Otherwise, it will clear the programmer.
    Clear,
    /// Select an object in the show.
    Select(Option<Object>),
}

impl Command {
    /// Parse a command from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use backstage::cmd::{Command, Object};
    /// # use backstage::show::FixtureId;
    ///
    /// let command = Command::parse("clear").unwrap();
    /// assert_eq!(command, Command::Clear);
    ///
    /// let command = Command::parse("select fixture 1").unwrap();
    /// assert_eq!(command, Command::Select(Some(Object::Fixture(Some(FixtureId::new(1))))));
    /// ```
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
    /// Execute a command in the show.
    ///
    /// # Examples
    ///
    /// ```
    /// # use backstage::cmd::{Command, Object};
    /// # use backstage::show::Show;
    ///
    /// let mut show = Show::new();
    /// show.execute_command(&Command::Clear).unwrap();
    /// ```
    pub fn execute_command(&mut self, command: &Command) -> Result<(), Error> {
        match command {
            Command::Clear => {
                if self.programmer().selected_fixture_ids().is_empty() {
                    self.programmer_mut().clear_changes()
                } else {
                    self.programmer_mut().clear_selection();
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

                    if !self.programmer().selected_fixture_ids().contains(id) {
                        self.programmer_mut().select_fixture(*id);
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
                Some(Object::Preset(filter, id)) => {
                    let Some(filter) = filter else {
                        return Err(Error::ExecutionError(
                            "No preset filter provided".to_string(),
                        ));
                    };

                    let Some(id) = id else {
                        return Err(Error::ExecutionError("No preset id provided".to_string()));
                    };

                    let preset = self
                        .data()
                        .preset(&filter, *id)
                        .ok_or_else(|| {
                            Error::ExecutionError(format!(
                                "Preset of type '{filter}' with id '{id}' not found"
                            ))
                        })?
                        .clone();

                    for fixture_id in self.programmer().selected_fixture_ids().to_vec() {
                        let Some(fixture) = self.patchlist().fixture(&fixture_id).cloned() else {
                            log::warn!("Failed to get fixture with id {fixture_id} while selecting preset {id}");
                            continue;
                        };

                        for (attribute_name, value) in &preset.attribute_values {
                            self.programmer_mut()
                                .changes_mut()
                                .set_attribute_value(
                                    &fixture,
                                    attribute_name.clone(),
                                    value.clone(),
                                )
                                .ok();
                        }
                    }
                }
                Some(Object::Effect(id)) => {
                    let Some(id) = id else {
                        return Err(Error::ExecutionError("No effect id provided".to_string()));
                    };

                    let effect = self
                        .data()
                        .effect(*id)
                        .ok_or_else(|| {
                            Error::ExecutionError(format!("Effect with id '{id}' not found"))
                        })?
                        .clone();

                    self.programmer_mut().set_selected_effect(Some(effect));
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
/// Represents an error that occurred during parsing or execution of a command.
pub enum Error {
    /// An unexpected token was encountered.
    #[error("Unexpected token: {0}")]
    UnexpectedToken(Token),
    /// An expected number was not found.
    #[error("Expected number")]
    ExpectedNumber,
    /// An expected string was not found.
    #[error("Expected string")]
    ExpectedString,
    /// An unexpected end of input was encountered.
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    /// An expected token was not found.
    #[error("Expected token: {0}")]
    ExpectedToken(Token),
    /// An execution error occurred.
    #[error("Execution error: {0}")]
    ExecutionError(String),
    /// A tokenization error occurred.
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
