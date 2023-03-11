# yad_semver (Yet Another Damn Semver)

This crate provides a simple [SemVer 2.0](https://semver.org/spec/v2.0.0.html) implementation.

SemVer structs can be converted to/from strings, and can be compared.

This crate exists because the "semver" crate is "for Cargo's flavor of Semantic Versioning",
whereas this crate structly follows the semver 2.0 specification.

## Usage

```rust
use yad_semver::SemVer;

// You can create SemVer structs in place
let v1 = SemVer::new(1, 0, 0, None, None);

// Or from strings
let v2 = "2.0.0-alpha".parse::<SemVer>().unwrap();

// SemVers can be compared and displayed
use std::cmp::max;
println!("The newest version is {}", max(v1, v2));
```