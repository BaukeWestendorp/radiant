use crate::command::Span;

use self::token::{Token, TokenKind};

pub mod rule;
pub mod token;

pub struct Lexer<'input> {
    input: &'input str,
    position: usize,
    eol: bool,
    rules: Vec<rule::Rule>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            position: 0,
            eol: false,
            rules: rule::get_rules(),
        }
    }

    fn next_token(&mut self, input: &str) -> Token {
        self.valid_token(input)
            .unwrap_or_else(|| self.invalid_token(input))
    }

    /// Returns `None` if the lexer cannot find a token at the start of `input`.
    fn valid_token(&mut self, input: &str) -> Option<Token> {
        let next = input.chars().next().unwrap();
        let (len, kind) = if next.is_whitespace() {
            (
                input
                    .char_indices()
                    .take_while(|(_, c)| c.is_whitespace())
                    .last()
                    .unwrap() // we know there is at least one whitespace character
                    .0 as usize
                    + 1,
                TokenKind::Whitespace,
            )
        } else {
            self.rules
                .iter()
                // `max_by_key` returns the last element if multiple rules match,
                // but we want earlier rules to "win" against later ones
                .rev()
                .filter_map(|rule| Some(((rule.matches)(input)?, rule.kind)))
                .max_by_key(|&(len, _)| len)?
        };

        let start = self.position;
        self.position += len;
        Some(Token {
            kind,
            span: Span {
                start,
                end: start + len,
            },
        })
    }

    /// Always "succeeds", because it creates an error `Token`.
    fn invalid_token(&mut self, input: &str) -> Token {
        let start = self.position;
        let len = input
            .char_indices()
            .find(|(pos, _)| self.valid_token(&input[*pos..]).is_some())
            .map(|(pos, _)| pos)
            .unwrap_or_else(|| input.len());
        debug_assert!(len <= input.len());

        // Because `valid_token` advances our position,
        // we need to reset it to after the erroneous token.
        self.position = start + len;
        Token {
            kind: TokenKind::Invalid,
            span: Span {
                start,
                end: start + len,
            },
        }
    }

    fn token(&mut self, kind: TokenKind, len: usize) -> Token {
        let token = Token {
            kind,
            span: Span {
                start: self.position,
                end: self.position + len,
            },
        };

        assert!(
            self.position + len <= self.input.len(),
            "Token out of bounds: {:?}. Input length: {}",
            token,
            self.input.len()
        );

        self.position += len;
        token
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.input.len() {
            if self.eol {
                return None;
            }
            self.eol = true;
            Some(self.token(TokenKind::EndOfLine, 0))
        } else {
            Some(self.next_token(&self.input[self.position..]))
        }
    }
}
