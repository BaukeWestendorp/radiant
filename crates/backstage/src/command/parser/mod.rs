use std::iter::Peekable;

use super::lexer::token::{Token, TokenKind};
use super::lexer::Lexer;

pub mod instructions;

pub struct TokenIter<'input> {
    lexer: Lexer<'input>,
}

impl<'input> TokenIter<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }
}

impl<'input> Iterator for TokenIter<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_token = self.lexer.next()?;
            if !matches!(next_token.kind, TokenKind::Whitespace) {
                return Some(next_token);
            }
        }
    }
}

pub struct Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    input: &'input str,
    tokens: Peekable<I>,
}

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    /// Get the source text of a token.
    pub fn text(&self, token: &'input Token) -> &'input str {
        token.text(&self.input)
    }

    /// Look-ahead one token and see what kind of token it is.
    pub fn peek(&mut self) -> TokenKind {
        self.tokens
            .peek()
            .map(|token| token.kind)
            .unwrap_or(TokenKind::EndOfLine)
    }

    /// Get the next token.
    pub fn next(&mut self) -> ParserResult<Token> {
        match self.tokens.next() {
            Some(token) => Ok(token),
            None => Err(ParserError {
                kind: ParserErrorKind::UnexpectedEndOfLine,
                token: None,
            }),
        }
    }

    /// Move forward one token in the input and check
    /// that we pass the kind of token we expect.
    pub fn consume(&mut self, expected: TokenKind) -> ParserResult<Token> {
        let token = self.next()?;

        if token.kind != expected {
            return Err(ParserError {
                kind: ParserErrorKind::UnexpectedToken,
                token: Some(token),
            });
        }

        Ok(token)
    }
}

impl<'input> Parser<'input, TokenIter<'input>> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            tokens: TokenIter::new(input).peekable(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub token: Option<Token>,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix = match &self.token {
            Some(token) => format!(": {} (at char {})", token.kind, token.span.start),
            None => "".to_string(),
        };
        match self.kind {
            ParserErrorKind::UnexpectedToken => write!(f, "Unexpected token{}", suffix),
            ParserErrorKind::UnexpectedEndOfLine => write!(f, "Unexpected end of line{}", suffix),
            ParserErrorKind::ExpectedId => write!(f, "Expected id{}", suffix),
            ParserErrorKind::ExpectedObject => write!(f, "Expected object{}", suffix),
            ParserErrorKind::InvalidId => write!(f, "Invalid id{}", suffix),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParserErrorKind {
    UnexpectedToken,
    UnexpectedEndOfLine,
    ExpectedId,
    ExpectedObject,
    InvalidId,
}

pub type ParserResult<T> = Result<T, ParserError>;
