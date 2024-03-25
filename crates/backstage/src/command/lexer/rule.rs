use lazy_static::lazy_static;
use regex::Regex;

use super::token::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rule {
    pub kind: TokenKind,
    pub matches: fn(&str) -> Option<usize>,
}

fn match_keyword(input: &str, keyword: &str) -> Option<usize> {
    input
        .to_ascii_lowercase()
        .starts_with(keyword.to_ascii_lowercase().as_str())
        .then_some(keyword.len())
}

fn match_regex(input: &str, r: &Regex) -> Option<usize> {
    r.find(input).map(|regex_match| regex_match.end())
}

lazy_static! {
    static ref INT_REGEX: Regex = Regex::new(r"^\d+").unwrap();
}

pub fn get_rules() -> Vec<Rule> {
    macro_rules! keyword {
        ($kind:expr) => {
            Rule {
                kind: $kind,
                matches: |input| match_keyword(input, $kind.to_string().as_str()),
            }
        };
    }

    macro_rules! regex {
        ($kind:expr, $regex:expr) => {
            Rule {
                kind: $kind,
                matches: |input| match_regex(input, $regex),
            }
        };
    }

    vec![
        keyword!(TokenKind::Clear),
        keyword!(TokenKind::Select),
        keyword!(TokenKind::Go),
        keyword!(TokenKind::Top),
        keyword!(TokenKind::Group),
        keyword!(TokenKind::Fixture),
        keyword!(TokenKind::Executor),
        regex!(TokenKind::Number, &INT_REGEX),
    ]
}
