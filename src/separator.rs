use std::fmt::Display;

use crate::errors::ParseError;

const DOT: char = '.';
const HYPHEN: char = '-';

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Separator {
    Dot,
    Hyphen,
}

impl TryFrom<char> for Separator {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            DOT => Ok(Separator::Dot),
            HYPHEN => Ok(Separator::Hyphen),
            _ => Err(ParseError::UnsupportedSeparator(value)),
        }
    }
}

impl Display for Separator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Separator::Dot => DOT,
            Separator::Hyphen => HYPHEN,
        };
        write!(f, "{}", val)
    }
}
