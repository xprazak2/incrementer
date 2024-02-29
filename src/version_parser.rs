use crate::{
    errors::{ParseError, ParseResult},
    separator::Separator,
    version::Version,
    version_buff::{VersionBuff, VersionBuffKind},
};

pub enum ParserState {
    Init,
    ParsingPrefix,
    ParsingMajor,
    ParsingMinor,
    ParsingPatch,
    ParsingSuffix,
}

pub struct VersionParser {
    state: ParserState,
    prefix: Vec<char>,
    suffix: Vec<char>,
    major: Vec<char>,
    minor: VersionBuff,
    patch: Vec<char>,
    sep: Option<Separator>,
}

impl VersionParser {
    fn new() -> Self {
        Self {
            state: ParserState::Init,
            prefix: vec![],
            suffix: vec![],
            major: vec![],
            minor: VersionBuff::new(),
            patch: vec![],
            sep: None,
        }
    }

    /// Parse a version.
    ///
    /// # Examples
    ///
    /// ```
    /// use incrementer::version_parser::VersionParser;
    ///
    /// let version = VersionParser::parse("55.69.1-pre.5").unwrap();
    /// ```
    pub fn parse(input: &str) -> ParseResult<Version> {
        let mut parser = Self::new();

        parser.parse_inner(input)
    }

    fn parse_inner(&mut self, input: &str) -> ParseResult<Version> {
        // It might be useful to report a character position if an error happened
        for item in input.chars() {
            match self.state {
                ParserState::Init => self.parse_init_state(item)?,
                ParserState::ParsingPrefix => self.parse_prefix_state(item),
                ParserState::ParsingMajor => self.parse_major_state(item)?,
                ParserState::ParsingMinor => self.parse_minor_state(item)?,
                ParserState::ParsingPatch => self.parse_patch_state(item)?,
                ParserState::ParsingSuffix => self.parse_suffix_state(item),
            }
        }

        match self.state {
            ParserState::Init | ParserState::ParsingPrefix => return Err(ParseError::InvalidInput),
            _ => (),
        }

        let major = self.to_num(&self.major)?;
        let minor = self.minor.try_parse()?;
        let patch = self.to_num(&self.patch)?;

        let sep = self.sep.as_ref().unwrap_or(&Separator::Dot);

        let prefix: String = self.prefix.iter().collect();
        let suffix: String = self.suffix.iter().collect();

        Ok(Version::new(
            major,
            &minor,
            patch,
            sep.to_owned(),
            &prefix,
            &suffix,
        ))
    }

    fn to_num(&self, item: &Vec<char>) -> ParseResult<usize> {
        if item.is_empty() {
            return Ok(0);
        }

        let mj: String = item.iter().collect();
        let rs = mj.parse::<usize>();
        rs.map_err(|_| ParseError::InvalidValue)
    }

    fn parse_init_state(&mut self, item: char) -> ParseResult<()> {
        if item.is_numeric() {
            self.state = ParserState::ParsingMajor;
            self.major.push(item);
        } else {
            self.state = ParserState::ParsingPrefix;
            self.prefix.push(item);
        }

        Ok(())
    }

    fn parse_prefix_state(&mut self, item: char) {
        if item.is_numeric() {
            self.state = ParserState::ParsingMajor;
            self.major.push(item);
        } else {
            self.prefix.push(item);
        }
    }

    fn parse_major_state(&mut self, item: char) -> ParseResult<()> {
        if item.is_numeric() {
            self.major.push(item);
            return Ok(());
        }

        let sep = Separator::try_from(item)?;
        self.state = ParserState::ParsingMinor;
        self.sep = Some(sep);
        Ok(())
    }

    fn parse_minor_state(&mut self, item: char) -> ParseResult<()> {
        if item.is_numeric() {
            self.minor.push(item, VersionBuffKind::Numeric)?;
            return Ok(());
        }

        if item.is_alphabetic() {
            self.minor.push(item, VersionBuffKind::Alphabetic)?;
            return Ok(());
        }

        if self.minor.is_empty() {
            return Err(ParseError::MinorError);
        }

        let sep = Separator::try_from(item)?;

        if Some(sep) != self.sep {
            self.state = ParserState::ParsingSuffix;
            self.suffix.push(item);
            return Ok(());
        }

        self.state = ParserState::ParsingPatch;
        Ok(())
    }

    fn parse_patch_state(&mut self, item: char) -> ParseResult<()> {
        if item.is_numeric() {
            self.patch.push(item);
            return Ok(());
        }

        if item.is_alphabetic() {
            self.patch.push(item);
            return Ok(());
        }

        if self.patch.is_empty() {
            return Err(ParseError::PatchError);
        }

        self.suffix.push(item);
        self.state = ParserState::ParsingSuffix;
        Ok(())
    }

    fn parse_suffix_state(&mut self, item: char) {
        // handle invalid semver suffixes?
        self.suffix.push(item);
    }
}
