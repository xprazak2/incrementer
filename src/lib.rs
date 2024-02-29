use errors::ParseResult;
use version_parser::VersionParser;

pub mod errors;
mod separator;
pub mod version;
mod version_buff;
mod version_parser;

/// Bump a patch version.
///
/// # Examples
///
/// ```
/// use incrementer::bump_patch;
///
/// let res = bump_patch("55.69.1-pre.5");
/// ```
pub fn bump_patch(input: &str) -> ParseResult<String> {
    let mut ver = VersionParser::parse(input)?;
    ver.bump_patch();
    Ok(ver.to_string())
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use crate::{bump_patch, errors::ParseError, VersionParser};

    #[test]
    fn should_parse_semver() {
        let res = VersionParser::parse("1.22.333").expect("should parse semver");
        assert_eq!(1, res.major());
        assert_eq!("22", res.minor());
        assert_eq!(333, res.patch());

        let res = VersionParser::parse("333.0.1").expect("should parse semver");
        assert_eq!(333, res.major());
        assert_eq!("0", res.minor());
        assert_eq!(1, res.patch());
    }

    #[test]
    fn should_parse_partial() {
        let res = VersionParser::parse("12.22").expect("should parse major and minor");
        assert_eq!(12, res.major());
        assert_eq!("22", res.minor());
        assert_eq!(0, res.patch());
        assert_eq!("12.22.0", res.semver());

        let res = VersionParser::parse("3").expect("should parse major only");
        assert_eq!(3, res.major());
        assert_eq!("0", res.minor());
        assert_eq!(0, res.patch());
        assert_eq!("3.0.0", res.semver());
    }

    #[test]
    fn should_parse_with_prefix_and_suffix() {
        let res = VersionParser::parse("v11.22.33-alpha.1").expect("should parse prefix");
        assert_eq!(11, res.major());
        assert_eq!("22", res.minor());
        assert_eq!(33, res.patch());
        assert_eq!("11.22.33-alpha.1", res.semver());

        let res =
            VersionParser::parse("release-11.22.33-alpha.1").expect("should parse long prefix");
        assert_eq!(11, res.major());
        assert_eq!("22", res.minor());
        assert_eq!(33, res.patch());
        assert_eq!("11.22.33-alpha.1", res.semver());
    }

    #[test]
    fn should_parse_hyphenated() {
        let res = VersionParser::parse("1-22-333").expect("should parse hyphenated");
        assert_eq!(1, res.major());
        assert_eq!("22", res.minor());
        assert_eq!(333, res.patch());

        let res = VersionParser::parse("whatever-1-22-333-pre-1").expect("should parse hyphenated");
        assert_eq!(1, res.major());
        assert_eq!("22", res.minor());
        assert_eq!(333, res.patch());
    }

    #[test]
    fn should_bump_patch_version() {
        assert_eq!(
            "release-v13.15.8-alpha.9",
            bump_patch("release-v13.15.7-alpha.9").expect("should bump patch")
        );
        assert_eq!("1.2.1", bump_patch("1.2").expect("should bump patch"));
    }

    #[test]
    fn should_parse_completely_custom_version_format() {
        let res = VersionParser::parse("2021-12.15-1").expect("should parse inverted separators");
        assert_eq!(2021, res.major());
        assert_eq!("12", res.minor());
        assert_eq!(0, res.patch());

        let res =
            VersionParser::parse("2023-Nov-02-v1").expect("should parse minor as month string");
        assert_eq!(2023, res.major());
        assert_eq!("Nov", res.minor());
        assert_eq!(2, res.patch());
    }

    #[test]
    fn should_bump_patch_for_all_fixtures() {
        let mut path = env::current_dir().expect("should get current dir");
        let mut exp_path = path.clone();
        path.push("Versions");
        exp_path.push("Versions-expected");

        let content = fs::read_to_string(path).expect("should read fixtures file");
        let split: Vec<_> = content
            .split(",")
            .into_iter()
            .map(|item| item.trim())
            .collect();

        let exp_content = fs::read_to_string(exp_path).expect("should read expected fixtures");
        let exp_split: Vec<_> = exp_content
            .split("\n")
            .into_iter()
            .map(|item| item.trim())
            .collect();

        for (idx, item) in split.iter().enumerate() {
            let res = bump_patch(item).expect("should bump patch");
            assert_eq!(exp_split[idx], res);
        }
    }

    #[test]
    fn should_error_on_unrecognized_formats() {
        assert_eq!(Err(ParseError::InvalidCharacter('a')), bump_patch("1.2a"));
        assert_eq!(Err(ParseError::InvalidValue), bump_patch("1.a"));
        assert_eq!(Err(ParseError::UnsupportedSeparator('a')), bump_patch("1a"));
        assert_eq!(Err(ParseError::InvalidInput), bump_patch("."));
        assert_eq!(Err(ParseError::InvalidInput), bump_patch("-."));
        assert_eq!(Err(ParseError::InvalidInput), bump_patch("a."));
        assert_eq!(
            Err(ParseError::UnsupportedSeparator('a')),
            bump_patch("a.1a.a")
        );
    }

    #[test]
    fn should_test_questionable_cases() {
        assert_eq!("2.1.1", bump_patch("2.1.").expect("should be ok"));
        assert_eq!("2.1.2.4", bump_patch("2.1.1.4").expect("should be ok"));
        assert_eq!(
            "2023-Nov-3-v1",
            bump_patch("2023-Nov-02-v1").expect("should be ok")
        );
        assert_eq!(
            "2021-12-1.15-1",
            bump_patch("2021-12.15-1").expect("should be ok")
        );
    }
}
