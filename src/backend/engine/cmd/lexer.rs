use std::fmt;

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
    type Item = Token<'src>;

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
            '[' => Some(Token::OpenBracket),
            ']' => Some(Token::CloseBracket),
            '(' => Some(Token::OpenParen),
            ')' => Some(Token::CloseParen),
            ',' => Some(Token::Comma),
            ':' => Some(Token::Colon),
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
                }
                Some(Token::String(content))
            }
            '0'..='9' => {
                self.position -= 1;

                // Parse number
                let start = self.position;
                let mut has_decimal = false;

                while self.position < self.source.len() {
                    let char = self.source.chars().nth(self.position).unwrap();
                    match char {
                        '0'..='9' => self.position += 1,
                        '.' if !has_decimal => {
                            has_decimal = true;
                            self.position += 1;
                        }
                        _ => break,
                    }
                }

                let num_str = &self.source[start..self.position];
                if has_decimal {
                    let num = num_str.parse::<f64>().expect("Should be able to parse f64");
                    Some(Token::Float(num))
                } else {
                    let num = num_str.parse::<i64>().expect("Should be able to parse i64");
                    Some(Token::Integer(num))
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
                Some(Token::Ident(ident))
            }
            invalid => Some(Token::Invalid(invalid)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use super::Token;

    #[test]
    fn test_lexer_empty_input() {
        let lexer = Lexer::new("");
        let tokens: Vec<Token> = lexer.collect();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_lexer_simple_tokens() {
        let lexer = Lexer::new("[ ] ( ) , :");
        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0], Token::OpenBracket);
        assert_eq!(tokens[1], Token::CloseBracket);
        assert_eq!(tokens[2], Token::OpenParen);
        assert_eq!(tokens[3], Token::CloseParen);
        assert_eq!(tokens[4], Token::Comma);
        assert_eq!(tokens[5], Token::Colon);
    }

    #[test]
    fn test_lexer_identifiers() {
        let lexer = Lexer::new("create executor sequence");
        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(tokens.len(), 3);
        assert!(matches!(tokens[0], Token::Ident("create")));
        assert!(matches!(tokens[1], Token::Ident("executor")));
        assert!(matches!(tokens[2], Token::Ident("sequence")));
    }

    #[test]
    fn test_lexer_numbers() {
        let lexer = Lexer::new("0 123 3.14");
        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Integer(0));
        assert_eq!(tokens[1], Token::Integer(123));
        assert_eq!(tokens[2], Token::Float(3.14));
    }

    #[test]
    fn test_lexer_strings() {
        let lexer = Lexer::new("\"Example Executor\" \"Basic Cue\"");
        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::String("Example Executor"));
        assert_eq!(tokens[1], Token::String("Basic Cue"));
    }

    #[test]
    fn test_lexer_complex_input() {
        let input = "create executor 0 \"Example Executor\"";
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Ident("create"));
        assert_eq!(tokens[1], Token::Ident("executor"));
        assert_eq!(tokens[2], Token::Integer(0));
        assert_eq!(tokens[3], Token::String("Example Executor"));
    }

    #[test]
    fn test_invalid_token() {
        let lexer = Lexer::new("@#$%^");
        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], Token::Invalid('@')));
        assert!(matches!(tokens[1], Token::Invalid('#')));
        assert!(matches!(tokens[2], Token::Invalid('$')));
        assert!(matches!(tokens[3], Token::Invalid('%')));
        assert!(matches!(tokens[4], Token::Invalid('^')));

        let lexer = Lexer::new("valid_token @ 123");
        let tokens: Vec<Token> = lexer.collect();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Ident("valid_token"));
        assert_eq!(tokens[1], Token::Invalid('@'));
        assert_eq!(tokens[2], Token::Integer(123));
    }
}
