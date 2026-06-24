# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changes

- Update nix flake using dependabot ([#33](https://github.com/vbrandl/badgers/pull/33))

### Dependencies
- Bump `actions/cache` from 4 to 6 ([#19](https://github.com/vbrandl/badgers/pull/19), [#61](https://github.com/vbrandl/badgers/pull/61))
- Bump `thiserror` from 2.0.17 to 2.0.18 ([#20](https://github.com/vbrandl/badgers/pull/20))
- Bump `actions/cache` from 4 to 5 ([#19](https://github.com/vbrandl/badgers/pull/19))
- Bump `actions/checkout` from 5 to 7 ([#18](https://github.com/vbrandl/badgers/pull/18), [#60](https://github.com/vbrandl/badgers/pull/60))
- Bump `rust-toolchain` from 1.91.0 to 1.96.0 ([#22](https://github.com/vbrandl/badgers/pull/22), [#25](https://github.com/vbrandl/badgers/pull/25), [#28](https://github.com/vbrandl/badgers/pull/28), [#30](https://github.com/vbrandl/badgers/pull/30), [#31](https://github.com/vbrandl/badgers/pull/31), [#38](https://github.com/vbrandl/badgers/pull/38), [#53](https://github.com/vbrandl/badgers/pull/53))
- Use rustsec audit action ([#36](https://github.com/vbrandl/badgers/pull/36))
- Bump `nixpkgs` from `dd9b079` to `567a49d` ([#34](https://github.com/vbrandl/badgers/pull/34), [#39](https://github.com/vbrandl/badgers/pull/39), [#42](https://github.com/vbrandl/badgers/pull/42), [#44](https://github.com/vbrandl/badgers/pull/44), [#46](https://github.com/vbrandl/badgers/pull/46), [#48](https://github.com/vbrandl/badgers/pull/48), [#51](https://github.com/vbrandl/badgers/pull/51), [#55](https://github.com/vbrandl/badgers/pull/55), [#57](https://github.com/vbrandl/badgers/pull/57), [#59](https://github.com/vbrandl/badgers/pull/59))
- Bump `rust-overlay` from `a1ab5e8` to `b728601` ([#35](https://github.com/vbrandl/badgers/pull/35), [#37](https://github.com/vbrandl/badgers/pull/37), [#40](https://github.com/vbrandl/badgers/pull/40), [#41](https://github.com/vbrandl/badgers/pull/41), [#43](https://github.com/vbrandl/badgers/pull/43), [#45](https://github.com/vbrandl/badgers/pull/45), [#47](https://github.com/vbrandl/badgers/pull/47), [#50](https://github.com/vbrandl/badgers/pull/50), [#52](https://github.com/vbrandl/badgers/pull/52), [#54](https://github.com/vbrandl/badgers/pull/54), [#57](https://github.com/vbrandl/badgers/pull/57), [#58](https://github.com/vbrandl/badgers/pull/58), [#59](https://github.com/vbrandl/badgers/pull/59), [#62](https://github.com/vbrandl/badgers/pull/62))

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
