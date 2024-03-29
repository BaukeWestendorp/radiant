use crate::command::Span;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Functions
    Clear,
    Select,
    Go,
    Top,

    // Objects
    Group,
    PresetColor,
    Fixture,
    Executor,

    Number,

    Whitespace,
    Invalid,
    EndOfLine,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Clear => "Clear".to_string(),
            Self::Select => "Select".to_string(),
            Self::Go => "Go".to_string(),
            Self::Top => "Top".to_string(),

            Self::Group => "Group".to_string(),
            Self::PresetColor => "Preset::Color".to_string(),
            Self::Fixture => "Fixture".to_string(),
            Self::Executor => "Executor".to_string(),

            Self::Number => "number".to_string(),

            Self::Whitespace => "whitespace".to_string(),
            Self::Invalid => "invalid".to_string(),
            Self::EndOfLine => "eol".to_string(),
        };

        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn text<'input>(&'input self, input: &'input str) -> &str {
        &input[self.span.start..self.span.end]
    }
}
