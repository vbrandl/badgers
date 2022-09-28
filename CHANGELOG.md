# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
