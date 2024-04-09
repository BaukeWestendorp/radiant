use anyhow::{anyhow, Result};

use self::lexer::Token;
use crate::command::lexer::Lexer;
use crate::{Cue, Group, Show};

mod lexer;

// FIXME: We should deserialize this from a string by parsing.
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize)]
pub enum Object {
    Fixture(usize),
    Group(usize),
    Sequence(usize),
    Cue { sequence_id: usize, cue_ix: usize },
    PresetBeam(usize),
    PresetColor(usize),
    PresetDimmer(usize),
    PresetFocus(usize),
    PresetGobo(usize),
    PresetPosition(usize),
    PresetAll(usize),
    Executor(usize),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Fixture(id) => write!(f, "fixture {}", id),
            Object::Group(id) => write!(f, "group {}", id),
            Object::Sequence(id) => write!(f, "sequence {}", id),
            Object::Cue {
                sequence_id,
                cue_ix,
            } => write!(f, "cue {}.{}", sequence_id, cue_ix),
            Object::PresetBeam(id) => write!(f, "preset.beam {}", id),
            Object::PresetColor(id) => write!(f, "preset.color {}", id),
            Object::PresetDimmer(id) => write!(f, "preset.dimmer {}", id),
            Object::PresetFocus(id) => write!(f, "preset.focus {}", id),
            Object::PresetGobo(id) => write!(f, "preset.gobo {}", id),
            Object::PresetPosition(id) => write!(f, "preset.position {}", id),
            Object::PresetAll(id) => write!(f, "preset.all {}", id),
            Object::Executor(id) => write!(f, "executor {}", id),
        }
    }
}

// FIXME: We should deserialize this from a string by parsing.
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize)]
pub enum Command {
    Clear,
    Select(Option<Object>),
    Store(Option<Object>),
    Go(Option<Object>),
    Top(Option<Object>),
}

impl Command {
    pub fn parse(input: impl AsRef<str>) -> Result<Command> {
        let mut lexer = Lexer::new(input.as_ref());

        macro_rules! confirm_end_of_command {
            ($token:expr) => {
                if lexer.next_token()?.is_some() {
                    return Err(anyhow!("Unexpected token after {} command", $token));
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
                Token::Store => {
                    let object = parse_object(&mut lexer)?;
                    confirm_end_of_command!(token);
                    Command::Store(Some(object))
                }
                Token::Go => match parse_object(&mut lexer)? {
                    object @ Object::Executor(_) => {
                        confirm_end_of_command!(token);
                        Command::Go(Some(object))
                    }
                    object => {
                        return Err(anyhow!("Go command expects an executor, got {:?}", object))
                    }
                },
                Token::Top => {
                    let object = parse_object(&mut lexer)?;
                    confirm_end_of_command!(token);
                    Command::Top(Some(object))
                }
                other => return Err(anyhow!("Unexpected token: {:?}", other)),
            },
            None => return Err(anyhow!("Unexpected end of input")),
        };
        Ok(command)
    }
}

fn parse_object(lexer: &mut Lexer) -> Result<Object> {
    let object = match lexer.next_token()? {
        Some((Token::Fixture, _start, _end)) => {
            let (number_token, _start, _end) = lexer
                .next_token()?
                .ok_or_else(|| anyhow!("Expected number"))?;
            match number_token {
                Token::Number(number) => Object::Fixture(number),
                _ => return Err(anyhow!("Expected number")),
            }
        }
        Some((Token::Group, _start, _end)) => {
            let (number_token, _start, _end) = lexer
                .next_token()?
                .ok_or_else(|| anyhow!("Expected number"))?;
            match number_token {
                Token::Number(number) => Object::Group(number),
                _ => return Err(anyhow!("Expected number")),
            }
        }
        Some((Token::Sequence, _start, _end)) => {
            let (number_token, _start, _end) = lexer
                .next_token()?
                .ok_or_else(|| anyhow!("Expected number"))?;
            match number_token {
                Token::Number(number) => Object::Sequence(number),
                _ => return Err(anyhow!("Expected number")),
            }
        }
        Some((Token::Cue, _start, _end)) => {
            let (number_token_1, _start, _end) = lexer
                .next_token()?
                .ok_or_else(|| anyhow!("Expected number"))?;
            consume(lexer, Token::Period)?;
            let (number_token_2, _start, _end) = lexer
                .next_token()?
                .ok_or_else(|| anyhow!("Expected number"))?;
            match number_token_1 {
                Token::Number(sequence_id) => match number_token_2 {
                    Token::Number(cue_ix) => Object::Cue {
                        sequence_id,
                        cue_ix,
                    },
                    _ => return Err(anyhow!("Expected number")),
                },
                _ => return Err(anyhow!("Expected number")),
            }
        }
        Some((Token::Preset, _start, _end)) => {
            consume(lexer, Token::Period)?;
            let (type_token, _start, _end) = lexer
                .next_token()?
                .ok_or_else(|| anyhow!("Expected color or executor"))?;
            let (number_token, _start, _end) = lexer
                .next_token()?
                .ok_or_else(|| anyhow!("Expected number"))?;
            match number_token {
                Token::Number(number) => match type_token {
                    Token::Color => Object::PresetColor(number),
                    _ => return Err(anyhow!("Expected color or executor")),
                },
                _ => return Err(anyhow!("Expected number")),
            }
        }
        Some((Token::Executor, _start, _end)) => {
            let (number_token, _start, _end) = lexer
                .next_token()?
                .ok_or_else(|| anyhow!("Expected number"))?;
            match number_token {
                Token::Number(number) => Object::Executor(number),
                _ => return Err(anyhow!("Expected number")),
            }
        }
        _ => return Err(anyhow!("Unexpected token")),
    };
    Ok(object)
}

fn consume(lexer: &mut Lexer, expected: Token) -> Result<()> {
    let (token, _start, _end) = lexer
        .next_token()?
        .ok_or_else(|| anyhow!("Expected {:?}", expected))?;
    if token != expected {
        return Err(anyhow!("Expected {:?}", expected));
    }
    Ok(())
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Command::Clear => write!(f, "clear"),
            Command::Select(Some(object)) => write!(f, "select {}", object),
            Command::Select(None) => write!(f, "select"),
            Command::Store(Some(object)) => write!(f, "store {}", object),
            Command::Store(None) => write!(f, "store"),
            Command::Go(Some(object)) => write!(f, "go {}", object),
            Command::Go(None) => write!(f, "go"),
            Command::Top(Some(object)) => write!(f, "top {}", object),
            Command::Top(None) => write!(f, "top"),
        }
    }
}

impl Show {
    pub fn execute_command(&mut self, command: &Command) -> Result<()> {
        match command {
            Command::Clear => {
                if self.programmer.selection.is_empty() {
                    self.programmer.changes.clear();
                } else {
                    self.programmer.selection.clear();
                }
            }
            Command::Select(object) => match object {
                Some(Object::Fixture(id)) => {
                    if !self.fixture_exists(*id) {
                        return Err(anyhow!("Fixture with id '{id}' not found"));
                    }

                    if !self.programmer.selection.contains(id) {
                        self.programmer.selection.push(*id);
                    } else {
                        log::debug!("Fixture with id '{id}' already selected");
                    }
                }
                Some(Object::Group(id)) => {
                    let group = self
                        .group(*id)
                        .ok_or_else(|| anyhow!("Group with id '{id}' not found"))?
                        .clone();
                    for fixture_id in group.fixtures.iter() {
                        self.execute_command(&Command::Select(Some(Object::Fixture(*fixture_id))))?;
                    }
                }
                Some(Object::PresetBeam(id)) => {
                    let beam = self
                        .beam_preset(*id)
                        .ok_or_else(|| anyhow!("Beam preset with id '{id}' not found"))?
                        .clone();
                    self.apply_preset(&beam)?;
                }
                Some(Object::PresetColor(id)) => {
                    let color_preset = self
                        .color_preset(*id)
                        .ok_or_else(|| anyhow!("Color preset with id '{id}' not found"))?
                        .clone();
                    self.apply_preset(&color_preset)?;
                }
                Some(Object::PresetDimmer(id)) => {
                    let dimmer_preset = self
                        .dimmer_preset(*id)
                        .ok_or_else(|| anyhow!("Dimmer preset with id '{id}' not found"))?
                        .clone();
                    self.apply_preset(&dimmer_preset)?;
                }
                Some(Object::PresetFocus(id)) => {
                    let focus_preset = self
                        .focus_preset(*id)
                        .ok_or_else(|| anyhow!("Focus preset with id '{id}' not found"))?
                        .clone();
                    self.apply_preset(&focus_preset)?;
                }
                Some(Object::PresetGobo(id)) => {
                    let gobo_preset = self
                        .gobo_preset(*id)
                        .ok_or_else(|| anyhow!("Gobo preset with id '{id}' not found"))?
                        .clone();
                    self.apply_preset(&gobo_preset)?;
                }
                Some(Object::PresetPosition(id)) => {
                    let position_preset = self
                        .position_preset(*id)
                        .ok_or_else(|| anyhow!("Position preset with id '{id}' not found"))?
                        .clone();
                    self.apply_preset(&position_preset)?;
                }
                Some(Object::PresetAll(id)) => {
                    let all_preset = self
                        .all_preset(*id)
                        .ok_or_else(|| anyhow!("All preset with id '{id}' not found"))?
                        .clone();
                    self.apply_preset(&all_preset)?;
                }
                Some(other) => return Err(anyhow!("'{other}' can not be selected")),
                None => return Err(anyhow!("Select requires a target object")),
            },
            Command::Store(object) => match object {
                Some(Object::Group(id)) => {
                    let selected_fixtures = self.selected_fixtures();
                    let group = Group {
                        id: *id,
                        label: "New Group".to_string(),
                        fixtures: selected_fixtures.to_vec(),
                    };
                    if group.fixtures.is_empty() {
                        return Err(anyhow!("No fixtures selected"));
                    }
                    self.data.groups.push(group);
                }
                Some(Object::Sequence(id)) => {
                    todo!()
                }
                Some(Object::Cue {
                    sequence_id,
                    cue_ix,
                }) => {
                    let changes = self.programmer_changes().clone();
                    if let Some(sequence) = self
                        .data
                        .sequences
                        .iter_mut()
                        .find(|s| s.id == *sequence_id)
                    {
                        if *cue_ix >= sequence.cues.len() {
                            sequence.cues.push(Cue {
                                label: "New Cue".to_string(),
                                changes,
                            });
                        } else {
                            for (fixture_id, attribute_values) in changes.into_iter() {
                                match sequence.cues[*cue_ix].changes.get_mut(&fixture_id) {
                                    Some(cue_changes) => {
                                        for (attribute_name, attribute_value) in
                                            attribute_values.into_iter()
                                        {
                                            cue_changes.insert(attribute_name, attribute_value);
                                        }
                                    }
                                    None => {
                                        sequence.cues[*cue_ix]
                                            .changes
                                            .insert(fixture_id, attribute_values);
                                    }
                                }
                            }
                        }
                    }
                }
                Some(object) => return Err(anyhow!("'{object}' can not be stored")),
                None => return Err(anyhow!("Store requires a destination object")),
            },
            Command::Go(object) => match object {
                Some(Object::Executor(id)) => {
                    let executor = self
                        .executor(*id)
                        .ok_or_else(|| anyhow!("Executor with id '{id}' not found"))?;
                    executor.go(self)
                }
                Some(_) => return Err(anyhow!("Go can only be used with executors")),
                None => return Err(anyhow!("Go requires an executor")),
            },
            Command::Top(object) => match object {
                Some(Object::Executor(id)) => {
                    let executor = self
                        .executor(*id)
                        .ok_or_else(|| anyhow!("Executor with id '{id}' not found"))?;
                    executor.top(self)
                }
                Some(_) => return Err(anyhow!("Top can only be used with executors")),
                None => return Err(anyhow!("Top requires an executor")),
            },
        }

        self.recalculate_stage_output();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::command::{Command, Object};

    #[test]
    fn test_parse_case_insensitivity() {
        let expected = Command::Clear;

        assert_eq!(Command::parse("clear").unwrap(), expected);
        assert_eq!(Command::parse("Clear").unwrap(), expected);
        assert_eq!(Command::parse("CLEAR").unwrap(), expected);
    }

    #[test]
    fn test_parse_clear() {
        let expected = Command::Clear;

        assert_eq!(Command::parse("clear").unwrap(), expected);
        assert!(Command::parse("clear foobar").is_err());
        assert!(Command::parse("clear   foobar").is_err());
    }

    #[test]
    fn test_parse_select() {
        let expected = Command::Select(Some(Object::Group(42)));

        assert_eq!(Command::parse("select group 42").unwrap(), expected);
        assert!(Command::parse("select group 42  foobar").is_err());
        assert!(Command::parse("select foobar group 42").is_err());
        assert!(Command::parse("select group foobar 42").is_err());
    }

    #[test]
    fn test_parse_store() {
        let expected = Command::Store(Some(Object::Group(42)));

        assert_eq!(Command::parse("store group 42").unwrap(), expected);
        assert!(Command::parse("store group 42  foobar").is_err());
        assert!(Command::parse("store foobar group 42").is_err());
        assert!(Command::parse("store group foobar 42").is_err());
    }

    #[test]
    fn test_parse_preset() {
        let expected = Command::Select(Some(Object::PresetColor(42)));

        assert_eq!(Command::parse("select preset.color 42").unwrap(), expected);
        assert!(Command::parse("select preset.color 42 foobar").is_err());
        assert!(Command::parse("select preset..color 42").is_err());
        assert!(Command::parse("select foobar preset.color 42").is_err());
        assert!(Command::parse("select preset.color foobar 42").is_err());
    }

    #[test]
    fn test_parse_go() {
        let expected = Command::Go(Some(Object::Executor(42)));

        assert_eq!(Command::parse("go executor 42").unwrap(), expected);
        assert!(Command::parse("go executor 42  foobar").is_err());
        assert!(Command::parse("go foobar 42").is_err());
        assert!(Command::parse("go group 42").is_err());
        assert!(Command::parse("go executor foobar 42").is_err());
    }
}
