use thiserror::Error;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("encountered unexpected character: `{0}`")]
    InvalidCharacter(char),
    #[error("error detecting a version value")]
    InvalidValue,
    #[error("unsupported separator: `{0}`")]
    UnsupportedSeparator(char),
    #[error("error when parsing major version")]
    MajorError,
    #[error("error when parsing major version")]
    MinorError,
    #[error("error when parsing patch version")]
    PatchError,
    #[error("invalid input - unrecognized version format")]
    InvalidInput,
}
