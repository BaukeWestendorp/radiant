use super::Error;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub enum Token {
    // Functions
    Clear,
    Select,

    // Objects
    Fixture,
    Group,

    // Other
    Period,
    Number(usize),
    String(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Clear => write!(f, "clear"),
            Token::Select => write!(f, "select"),

            Token::Fixture => write!(f, "fixture"),
            Token::Group => write!(f, "group"),

            Token::Period => write!(f, "."),
            Token::Number(n) => write!(f, "{}", n),
            Token::String(s) => write!(f, "\"{}\"", s),
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
            remaining: input.to_string(),
        }
    }

    pub fn next_token(&mut self) -> Result<Option<(Token, usize, usize)>, Error> {
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

fn lex_single_token(input: &str) -> Result<(Token, usize), Error> {
    let next = match input.chars().next() {
        Some(char) => char,
        None => return Err(Error::TokenizationError("Unexpected EOF".to_string())),
    };

    let (token, len) = match next {
        '0'..='9' => tokenize_number(input)?,
        '"' => lex_string(input)?,
        c @ '_' | c if c.is_alphabetic() => lex_keyword(input)?,
        '.' => (Token::Period, 1),
        other => {
            return Err(Error::TokenizationError(format!(
                "Unexpected character: '{other}'"
            )))
        }
    };

    Ok((token, len))
}

fn lex_string(input: &str) -> Result<(Token, usize), Error> {
    let mut chars = input.chars();
    let mut len = 0;

    if chars.next() != Some('"') {
        return Err(Error::TokenizationError("Expected '\"'".to_string()));
    }

    let mut string = String::new();
    let mut escaped = false;

    for char in chars {
        len += char.len_utf8();

        if escaped {
            string.push(char);
            escaped = false;
        } else {
            match char {
                '\\' => escaped = true,
                '"' => {
                    len += 1;
                    break;
                }
                _ => string.push(char),
            }
        }
    }

    Ok((Token::String(string), len))
}

fn lex_keyword(input: &str) -> Result<(Token, usize), Error> {
    match input.chars().next() {
        Some(char) if char.is_ascii_digit() => {
            return Err(Error::TokenizationError(
                "Keywords cannot start with a number.".to_string(),
            ))
        }
        None => return Err(Error::TokenizationError("Unexpected EOF".to_string())),
        _ => {}
    }

    let (got, bytes_read) = take_while(input, |char| char == '_' || char.is_alphanumeric())?;

    let token = match got.to_lowercase().as_str() {
        "clear" => Token::Clear,
        "select" => Token::Select,

        "fixture" => Token::Fixture,
        "group" => Token::Group,

        other => {
            return Err(Error::TokenizationError(format!(
                "Unknown keyword: '{other}'"
            )))
        }
    };

    Ok((token, bytes_read))
}

fn tokenize_number(input: &str) -> Result<(Token, usize), Error> {
    let (number, bytes_read) = take_while(input, |char| char.is_ascii_digit())?;
    let n: usize = number
        .parse()
        .map_err(|_| Error::TokenizationError(format!("Failed to parse number: '{}'", number)))?;
    Ok((Token::Number(n), bytes_read))
}

fn take_while<F>(input: &str, mut predicate: F) -> Result<(&str, usize), Error>
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
        Err(Error::TokenizationError("No matches".to_string()))
    } else {
        Ok((&input[..ix], ix))
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::lexer::{lex_single_token, Token};

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
    fn test_lex_string() {
        let input = r#""hello""#;
        let (token, bytes_read) = lex_single_token(input).unwrap();
        assert_eq!(token, Token::String("hello".to_string()));
        assert_eq!(bytes_read, 7);
    }

    #[test]
    fn test_escaped_string() {
        let input = r#""\"hello\"""#;
        let (token, bytes_read) = lex_single_token(input).unwrap();
        assert_eq!(token, Token::String(r#""hello""#.to_string()));
        assert_eq!(bytes_read, 11);
    }

    #[test]
    fn test_invalid_character() {
        let input = "$";
        let result = lex_single_token(input);
        assert!(result.is_err());
    }
}
