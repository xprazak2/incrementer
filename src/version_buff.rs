use crate::errors::{ParseError, ParseResult};

const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

#[derive(Eq, PartialEq)]
pub enum VersionBuffKind {
    Numeric,
    Alphabetic,
    Unknown,
}

pub struct VersionBuff {
    buff: Vec<char>,
    pub kind: VersionBuffKind,
}

impl VersionBuff {
    pub fn new() -> Self {
        Self {
            buff: vec![],
            kind: VersionBuffKind::Unknown,
        }
    }

    pub fn push(&mut self, item: char, kind: VersionBuffKind) -> ParseResult<()> {
        if self.kind == kind && self.kind != VersionBuffKind::Unknown {
            self.buff.push(item);
            return Ok(());
        }

        if self.kind == VersionBuffKind::Unknown {
            self.kind = kind;
            self.buff.push(item);
            return Ok(());
        }

        Err(ParseError::InvalidCharacter(item))
    }

    pub fn is_empty(&self) -> bool {
        self.buff.is_empty()
    }

    pub fn try_parse(&self) -> ParseResult<String> {
        match self.kind {
            VersionBuffKind::Numeric => {
                if self.buff.is_empty() {
                    return Ok("0".into());
                }

                let mj: String = self.buff.iter().collect();
                let rs = mj.parse::<usize>();
                match rs {
                    Ok(_) => Ok(mj),
                    Err(_) => Err(ParseError::InvalidValue),
                }
            }
            VersionBuffKind::Alphabetic => {
                let mj: String = self.buff.iter().collect();
                if MONTHS.contains(&mj.as_str()) {
                    Ok(mj)
                } else {
                    Err(ParseError::InvalidValue)
                }
            }
            VersionBuffKind::Unknown => Ok("0".into()),
        }
    }
}
