# Incrementer

A create that increments patch version for some non-semver version formats.

## Usage

Add as a dependency to your cargo.toml

```
incrementer = { path = "path/to/incrementer" }
```

then you are all set up:

```rust
use incrementer::{bump_patch, version_parser::VersionParser};

fn main() {
    // All in one
    println!("{}", bump_patch("v1-9-4").unwrap());
    // v1-9-5

    // Step by step
    let mut version = VersionParser::parse("v2-5-6").unwrap();
    println!("{}", version);
    // v2-5-6

    println!("{}", version.semver());
    // 2.5.6

    version.bump_patch();

    println!("{}", version);
    // v2-5-7

    println!("{}", version.semver());
    // 2.5.7
}
```

