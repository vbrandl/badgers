# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Dependencies

- Bump `actions/cache` from 4 to 5 ([#19](https://github.com/vbrandl/badgers/pull/19))

## [2.0.0] 2025-11-09

### Changes

- Use dedicated error type ([#17](https://github.com/vbrandl/badgers/pull/17))

### Dependencies
- Bump `base64` from 0.21 to 0.22 ([#8](https://github.com/vbrandl/badgers/pull/8))
- Bump `actions/cache` from 3 to 4 ([#10](https://github.com/vbrandl/badgers/pull/10))
- Bump `actions/checkout` from 3 to 5 ([#11](https://github.com/vbrandl/badgers/pull/11), [#15](https://github.com/vbrandl/badgers/pull/15))
- Bump `dangoslen/dependabot-changelog-helper` from 3 to 4 ([#13](https://github.com/vbrandl/badgers/pull/13))
- Bump `stefanzweifel/git-auto-commit-action` from 5 to 7 ([#14](https://github.com/vbrandl/badgers/pull/14), [#16](https://github.com/vbrandl/badgers/pull/16))

## [1.2.0] 2023-01-18

### Dependencies

* Updated the requirements on [base64](https://github.com/marshallpierce/rust-base64) from 0.20 to 0.21 to permit the latest version ([#5])

[#5]: https://github.com/vbrandl/badgers/pull/5


## [1.1.0] 2022-12-18

### Dependencies

* Updated the requirements on [base64](https://github.com/marshallpierce/rust-base64) from 0.13 to 0.20 to permit the latest version ([#2])

[#2]: https://github.com/vbrandl/badgers/pull/2


## [1.0.0] 2022-09-28

### Fixed

- [deprecated](https://rustsec.org/advisories/RUSTSEC-2021-0140.html) [`rusttype`](https://crates.io/crates/rusttype) has been replaced by [`ab_glyph`](https://crates.io/crates/ab_glyph)


## Point of Fork

Changes beyond this point were made pre-fork in the
[`badge`](https://crates.io/crates/badge) crate and are kept for historical
reasons.

## [0.3.0] 2020-07-26

### Changed

- `Badge::new()` will now return an error if the status or subject is empty
- `Badge::new()` now caches `DejaVuSans.ttf` the first time it is loaded,
  improving latency at a small memory cost.

### Fixed

- `rusttype` has been upgraded to `0.9`,
  removing the dependency on the deprecated crate `stb_truetype`
- `base64` has been upgraded to `0.12`
