# topiary

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/topiary)
[![Crates.io](https://img.shields.io/crates/v/topiary)](https://crates.io/crates/topiary)
[![Docs.rs](https://docs.rs/topiary/badge.svg)](https://docs.rs/crates/topiary)

![Rust 1.40.0](https://img.shields.io/static/v1?logo=Rust&label=&message=1.40.0&color=grey)
[![Build Status](https://travis-ci.com/Tamschi/topiary.svg?branch=develop)](https://travis-ci.com/Tamschi/topiary/branches)
![Crates.io - License](https://img.shields.io/crates/l/topiary/0.0.1)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/topiary)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/topiary)](https://github.com/Tamschi/topiary/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/topiary)](https://github.com/Tamschi/topiary/pulls)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/topiary.svg)](https://web.crev.dev/rust-reviews/crate/topiary/)

A very WIP and experimental CSS scoping library.

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add topiary
```

## Example

```rust
// TODO_EXAMPLE
```

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`topiary` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

* The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
* The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).