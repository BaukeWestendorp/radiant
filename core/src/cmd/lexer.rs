use std::fmt;

use crate::error::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'src> {
    Ident(&'src str),
    Integer(i64),
    Float(f64),
    String(&'src str),
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    Comma,
    Colon,
    Invalid(char),
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Ident(ident) => write!(f, "{}", ident),
            Token::Integer(num) => write!(f, "{}", num),
            Token::Float(num) => write!(f, "{}", num),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::OpenBracket => write!(f, "["),
            Token::CloseBracket => write!(f, "]"),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Invalid(c) => write!(f, "<invalid {}>", c),
        }
    }
}

pub struct Lexer<'src> {
    source: &'src str,
    position: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { source, position: 0 }
    }

    fn skip_whitespace(&mut self) {
        let remaining = &self.source[self.position..];
        let trimmed = remaining.trim_start();
        self.position += remaining.len() - trimmed.len();
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Result<Token<'src>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.source.len() {
            return None;
        }

        self.skip_whitespace();

        if self.position >= self.source.len() {
            return None;
        }

        let c = self.source.chars().nth(self.position).unwrap();
        self.position += 1;
        match c {
            '[' => Some(Ok(Token::OpenBracket)),
            ']' => Some(Ok(Token::CloseBracket)),
            '(' => Some(Ok(Token::OpenParen)),
            ')' => Some(Ok(Token::CloseParen)),
            ',' => Some(Ok(Token::Comma)),
            ':' => Some(Ok(Token::Colon)),
            '"' => {
                let start = self.position;
                while self.position < self.source.len()
                    && self.source.chars().nth(self.position).unwrap() != '"'
                {
                    self.position += 1;
                }
                let content = &self.source[start..self.position];
                if self.position < self.source.len() {
                    self.position += 1; // Skip closing quote.
                } else {
                    return Some(Err(eyre::eyre!("encountered unescaped string")));
                }
                Some(Ok(Token::String(content)))
            }
            '0'..='9' | '-' => {
                self.position -= 1;

                // Parse number
                let start = self.position;
                let mut has_decimal = false;

                while self.position < self.source.len() {
                    let char = self.source.chars().nth(self.position).unwrap();
                    match char {
                        '-' => {
                            if self.position == start {
                                self.position += 1;
                            } else {
                                return Some(Err(eyre::eyre!(
                                    "encountered unexpected '-' after number"
                                )));
                            }
                        }
                        '0'..='9' => self.position += 1,
                        '.' => {
                            if !has_decimal {
                                has_decimal = true;
                                self.position += 1;
                            } else {
                                return Some(Err(eyre::eyre!("unexpected '.' after number")));
                            }
                        }
                        _ => break,
                    }
                }

                let num_str = &self.source[start..self.position];
                if has_decimal {
                    let num = num_str.parse::<f64>().expect("should be able to parse f64");
                    Some(Ok(Token::Float(num)))
                } else {
                    let num = num_str.parse::<i64>().expect("should be able to parse i64");
                    Some(Ok(Token::Integer(num)))
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                self.position -= 1;

                // Parse identifier
                let start = self.position;

                while self.position < self.source.len() {
                    let char = self.source.chars().nth(self.position).unwrap();
                    match char {
                        'a'..='z' | 'A'..='Z' | '_' => self.position += 1,
                        _ => break,
                    }
                }

                let ident = &self.source[start..self.position];
                Some(Ok(Token::Ident(ident)))
            }
            invalid => Some(Ok(Token::Invalid(invalid))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};
    use crate::error::Result;

    #[test]
    fn lexer_empty_input() {
        let lexer = Lexer::new("");
        let tokens = lexer.collect::<Result<Vec<_>>>().unwrap();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn lexer_simple_tokens() {
        let lexer = Lexer::new("[ ] ( ) , :");
        let tokens = lexer.collect::<Result<Vec<_>>>().unwrap();

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0], Token::OpenBracket);
        assert_eq!(tokens[1], Token::CloseBracket);
        assert_eq!(tokens[2], Token::OpenParen);
        assert_eq!(tokens[3], Token::CloseParen);
        assert_eq!(tokens[4], Token::Comma);
        assert_eq!(tokens[5], Token::Colon);
    }

    #[test]
    fn lexer_identifiers() {
        let lexer = Lexer::new("create executor sequence CAPS");
        let tokens = lexer.collect::<Result<Vec<_>>>().unwrap();

        assert_eq!(tokens.len(), 4);
        assert!(matches!(tokens[0], Token::Ident("create")));
        assert!(matches!(tokens[1], Token::Ident("executor")));
        assert!(matches!(tokens[2], Token::Ident("sequence")));
        assert!(matches!(tokens[3], Token::Ident("CAPS")));
    }

    #[test]
    fn lexer_numbers() {
        let lexer = Lexer::new("0 123 3.14 -2.876");
        let tokens = lexer.collect::<Result<Vec<_>>>().unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Integer(0));
        assert_eq!(tokens[1], Token::Integer(123));
        assert_eq!(tokens[2], Token::Float(3.14));
        assert_eq!(tokens[3], Token::Float(-2.876));
    }

    #[test]
    fn lexer_invalid_numbers_with_minus() {
        let lexer = Lexer::new("0-2");
        assert!(lexer.collect::<Result<Vec<_>>>().is_err());
    }

    #[test]
    fn lexer_invalid_numbers_with_two_periods() {
        let lexer = Lexer::new("0.2.");
        assert!(lexer.collect::<Result<Vec<_>>>().is_err());
    }

    #[test]
    fn lexer_strings() {
        let lexer = Lexer::new("\"Example Executor\" \"Basic Cue\"");
        let tokens = lexer.collect::<Result<Vec<_>>>().unwrap();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::String("Example Executor"));
        assert_eq!(tokens[1], Token::String("Basic Cue"));
    }

    #[test]
    fn lexer_unescaped_string() {
        let lexer = Lexer::new("\"Example Exec");
        assert!(lexer.collect::<Result<Vec<_>>>().is_err());
    }

    #[test]
    fn lexer_complex_input() {
        let input = "create executor 0 \"Example Executor\"";
        let lexer = Lexer::new(input);
        let tokens = lexer.collect::<Result<Vec<_>>>().unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Ident("create"));
        assert_eq!(tokens[1], Token::Ident("executor"));
        assert_eq!(tokens[2], Token::Integer(0));
        assert_eq!(tokens[3], Token::String("Example Executor"));
    }

    #[test]
    fn lexer_invalid_token() {
        let lexer = Lexer::new("@#$%^");
        let tokens = lexer.collect::<Result<Vec<_>>>().unwrap();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::Invalid('@'));
        assert_eq!(tokens[1], Token::Invalid('#'));
        assert_eq!(tokens[2], Token::Invalid('$'));
        assert_eq!(tokens[3], Token::Invalid('%'));
        assert_eq!(tokens[4], Token::Invalid('^'));

        let lexer = Lexer::new("valid_token @ 123");
        let tokens = lexer.collect::<Result<Vec<_>>>().unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Ident("valid_token"));
        assert_eq!(tokens[1], Token::Invalid('@'));
        assert_eq!(tokens[2], Token::Integer(123));
    }

    #[test]
    fn lexer_token_to_string() {
        let lexer = Lexer::new(r#"ident 1 -3.14 "string" [ ] ( ) , : @"#);
        let tokens: Vec<_> = lexer.map(|token| token.unwrap().to_string()).collect();
        assert_eq!(
            tokens,
            vec!["ident", "1", "-3.14", r#""string""#, "[", "]", "(", ")", ",", ":", "<invalid @>"]
        );
    }
}
