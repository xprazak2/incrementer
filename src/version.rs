use std::fmt::Display;

use crate::separator::Separator;

#[derive(Debug)]
pub struct Version {
    prefix: String,
    suffix: String,
    major: usize,
    minor: String,
    patch: usize,
    sep: Separator,
}

impl Version {
    pub fn new(
        major: usize,
        minor: &str,
        patch: usize,
        sep: Separator,
        prefix: &str,
        suffix: &str,
    ) -> Self {
        Self {
            prefix: prefix.into(),
            suffix: suffix.into(),
            major,
            minor: minor.into(),
            patch,
            sep,
        }
    }

    pub fn major(&self) -> usize {
        self.major
    }

    pub fn minor(&self) -> &str {
        &self.minor
    }

    pub fn patch(&self) -> usize {
        self.patch
    }

    pub fn bump_patch(&mut self) {
        self.patch += 1;
    }

    pub fn semver(&self) -> String {
        format!(
            "{}.{}.{}{}",
            self.major, self.minor, self.patch, self.suffix
        )
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}",
            self.prefix, self.major, self.sep, self.minor, self.sep, self.patch, self.suffix
        )
    }
}
