use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub enum Token {
    // Functions
    Clear,
    Select,
    Store,
    Go,
    Top,

    // Objects
    Fixture,
    Group,
    Executor,

    Preset,
    Color,
    Seperator,

    // Other
    Number(usize),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Clear => write!(f, "clear"),
            Token::Select => write!(f, "select"),
            Token::Store => write!(f, "store"),
            Token::Go => write!(f, "go"),
            Token::Top => write!(f, "top"),
            Token::Fixture => write!(f, "fixture"),
            Token::Group => write!(f, "group"),
            Token::Executor => write!(f, "executor"),
            Token::Preset => write!(f, "preset"),
            Token::Color => write!(f, "color"),
            Token::Seperator => write!(f, "seperator"),
            Token::Number(n) => write!(f, "{}", n),
        }
    }
}

pub struct Lexer {
    ix: usize,
    remaining: String,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            ix: 0,
            remaining: input.to_lowercase(),
        }
    }

    pub fn next_token(&mut self) -> Result<Option<(Token, usize, usize)>> {
        self.skip_whitespace();

        if self.remaining.is_empty() {
            Ok(None)
        } else {
            let start = self.ix;
            let (token, bytes_read) = lex_single_token(self.remaining.as_str())?;
            self.consume(bytes_read);
            let end = self.ix;
            Ok(Some((token, start, end)))
        }
    }

    fn skip_whitespace(&mut self) {
        let skipped = skip(self.remaining.as_str());
        self.consume(skipped);
    }

    fn consume(&mut self, len: usize) {
        self.remaining = self.remaining[len..].to_string();
        self.ix += len;
    }
}

fn skip(input: &str) -> usize {
    let mut remaining = input;

    loop {
        let ws = skip_whitespace(remaining);
        remaining = &remaining[ws..];
        if ws == 0 {
            return input.len() - remaining.len();
        }
    }
}

fn skip_whitespace(input: &str) -> usize {
    match take_while(input, |ch| ch.is_whitespace()) {
        Ok((_, bytes_skipped)) => bytes_skipped,
        _ => 0,
    }
}

fn lex_single_token(input: &str) -> Result<(Token, usize)> {
    let next = match input.chars().next() {
        Some(char) => char,
        None => return Err(anyhow!("Unexpected EOF")),
    };

    let (token, len) = match next {
        '0'..='9' => tokenize_number(input)?,
        c @ '_' | c if c.is_alphabetic() => lex_keyword(input)?,
        ':' => (Token::Seperator, 1),
        other => return Err(anyhow!("Unexpected character: '{other}'")),
    };

    Ok((token, len))
}

fn lex_keyword(input: &str) -> Result<(Token, usize)> {
    match input.chars().next() {
        Some(char) if char.is_ascii_digit() => {
            return Err(anyhow!("Keywords cannot start with a number."))
        }
        None => return Err(anyhow!("Unexpected EOF")),
        _ => {}
    }

    let (got, bytes_read) = take_while(input, |char| char == '_' || char.is_alphanumeric())?;

    let token = match got {
        "clear" => Token::Clear,
        "select" => Token::Select,
        "store" => Token::Store,
        "go" => Token::Go,
        "top" => Token::Top,
        "fixture" => Token::Fixture,
        "group" => Token::Group,
        "executor" => Token::Executor,
        "preset" => Token::Preset,
        "color" => Token::Color,
        other => return Err(anyhow!("Unknown keyword: '{other}'")),
    };

    Ok((token, bytes_read))
}

fn tokenize_number(input: &str) -> Result<(Token, usize)> {
    let (number, bytes_read) = take_while(input, |char| char.is_ascii_digit())?;
    let n: usize = number.parse()?;
    Ok((Token::Number(n), bytes_read))
}

fn take_while<F>(input: &str, mut predicate: F) -> Result<(&str, usize)>
where
    F: FnMut(char) -> bool,
{
    let mut ix = 0;

    for char in input.chars() {
        let should_continue = predicate(char);

        if !should_continue {
            break;
        }

        ix += char.len_utf8();
    }

    if ix == 0 {
        Err(anyhow!("No matches"))
    } else {
        Ok((&input[..ix], ix))
    }
}

#[cfg(test)]
mod tests {
    use crate::command::lexer::{lex_single_token, Token};

    #[test]
    fn test_lex_number() {
        let input = "123";
        let (token, bytes_read) = lex_single_token(input).unwrap();
        assert_eq!(token, Token::Number(123));
        assert_eq!(bytes_read, 3);
    }

    #[test]
    fn test_lex_keyword() {
        let input = "clear";
        let (token, bytes_read) = lex_single_token(input).unwrap();
        assert_eq!(token, Token::Clear);
        assert_eq!(bytes_read, 5);
    }

    #[test]
    fn test_invalid_character() {
        let input = "$";
        let result = lex_single_token(input);
        assert!(result.is_err());
    }
}
