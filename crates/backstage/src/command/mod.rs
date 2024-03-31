use anyhow::{anyhow, Result};

use crate::command::lexer::Lexer;

use self::lexer::Token;

mod lexer;

// FIXME: We should deserialize this from a string by parsing.
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize)]
pub enum Object {
    Fixture(usize),
    Group(usize),
    PresetColor(usize),
    Executor(usize),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Fixture(id) => write!(f, "Fixture {}", id),
            Object::Group(id) => write!(f, "Group {}", id),
            Object::PresetColor(id) => write!(f, "Preset:Color {}", id),
            Object::Executor(id) => write!(f, "Executor {}", id),
        }
    }
}

// FIXME: We should deserialize this from a string by parsing.
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize)]
pub enum Command {
    Clear,
    Select(Object),
    Store(Object),
    Go(Object),
    Top(Object),
}

impl Command {
    pub fn parse(input: impl AsRef<str>) -> Result<Command> {
        let mut lexer = Lexer::new(input.as_ref());

        macro_rules! confirm_end_of_command {
            ($token_type:literal) => {
                if lexer.next_token()?.is_some() {
                    return Err(anyhow!("Unexpected token after {} command", $token_type));
                }
            };
        }

        let command = match lexer.next_token()? {
            Some((token, _start, _end)) => match token {
                Token::Clear => {
                    confirm_end_of_command!("Clear");
                    Command::Clear
                }
                Token::Select => {
                    let object = parse_object(&mut lexer)?;
                    confirm_end_of_command!("Select");
                    Command::Select(object)
                }
                Token::Store => {
                    let object = parse_object(&mut lexer)?;
                    confirm_end_of_command!("Store");
                    Command::Store(object)
                }
                Token::Go => match parse_object(&mut lexer)? {
                    object @ Object::Executor(_) => {
                        confirm_end_of_command!("Go");
                        Command::Go(object)
                    }
                    object => {
                        return Err(anyhow!("Go command expects an executor, got {:?}", object))
                    }
                },
                Token::Top => {
                    let object = parse_object(&mut lexer)?;
                    confirm_end_of_command!("Top");
                    Command::Top(object)
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
        Some((Token::Preset, _start, _end)) => {
            consume(lexer, Token::Seperator)?;
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
        let expected = Command::Select(Object::Group(42));

        assert_eq!(Command::parse("select group 42").unwrap(), expected);
        assert!(Command::parse("select group 42  foobar").is_err());
        assert!(Command::parse("select foobar group 42").is_err());
        assert!(Command::parse("select group foobar 42").is_err());
    }

    #[test]
    fn test_parse_store() {
        let expected = Command::Store(Object::Group(42));

        assert_eq!(Command::parse("store group 42").unwrap(), expected);
        assert!(Command::parse("store group 42  foobar").is_err());
        assert!(Command::parse("store foobar group 42").is_err());
        assert!(Command::parse("store group foobar 42").is_err());
    }

    #[test]
    fn test_parse_preset() {
        let expected = Command::Select(Object::PresetColor(42));

        assert_eq!(Command::parse("select preset:color 42").unwrap(), expected);
        assert!(Command::parse("select preset:color 42 foobar").is_err());
        assert!(Command::parse("select preset::color 42").is_err());
        assert!(Command::parse("select foobar preset:color 42").is_err());
        assert!(Command::parse("select preset:color foobar 42").is_err());
    }

    #[test]
    fn test_parse_go() {
        let expected = Command::Go(Object::Executor(42));

        assert_eq!(Command::parse("go executor 42").unwrap(), expected);
        assert!(Command::parse("go executor 42  foobar").is_err());
        assert!(Command::parse("go foobar 42").is_err());
        assert!(Command::parse("go group 42").is_err());
        assert!(Command::parse("go executor foobar 42").is_err());
    }
}
