# Incrementer

A create that increments patch version for some non-semver version formats.

## Usage

Add as a dependency to your cargo.toml

```
incrementer = { path = "path/to/incrementer" }
```

then you are all set up:

```rust
use incrementer::bump_patch;

fn main() {
    println!("{}", bump_patch("v1-9-4").unwrap());
}
```

