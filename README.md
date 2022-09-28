# Badgers - Create SVG Badges

[![Hits-of-Code](https://hitsofcode.com/github/vbrandl/badgers)](https://hitsofcode.com/github/vbrandl/badgers/view)
[![Rust](https://github.com/vbrandl/badgers/actions/workflows/ci.yml/badge.svg)](https://github.com/vbrandl/badgers/actions/workflows/ci.yml)
[![dependency status](https://deps.rs/repo/github/vbrandl/badgers/status.svg)](https://deps.rs/repo/github/vbrandl/badgers)
[![Crates.io](https://img.shields.io/crates/v/badgers.svg)](https://crates.io/crates/badgers)
[![docs.rs](https://docs.rs/badgers/badge.svg)](https://docs.rs/badgers/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/vbrandl/badgers/blob/master/LICENSE)

This is a Rust crate to generate simple SVG badges to be used e.g. in README
files. The subject, status and color of the badge can be configured.


## Example

```rust
use badgers::{BadgeOptions, Badge};

let options = BadgeOptions {
    subject: "docs".to_owned(),
    status: "0.5.3".to_owned(),
    color: "#4d76ae".to_owned(),
};
let badge = Badge::new(options).unwrap();
let svg = badge.to_svg();
```

## Origin Story

This is a fork of the [`badge`](https://crates.io/crates/badge) crate in order
to replace the now
[deprecated](https://rustsec.org/advisories/RUSTSEC-2021-0140.html)
[`rusttype`](https://crates.io/crates/rusttype) crate. `rusttype` was replaced
by [`ab_glyph`](https://crates.io/crates/ab_glyph) as suggested in the rustsec
advisory.

This fork was created since the original `badge` crate is no longer maintained
and the code has been removed from the originating repository:
[rust-lang/docs.rs@94f3bba](https://github.com/rust-lang/docs.rs/commit/94f3bba6815412bc4672621c4690a93e656486c7)

While trying to be a 1:1 replacement of `badge`, either my lack of deep
understanding of the problem space or minor differences between `rusttype` and
`ab_glyph` required some 1 pixel changes in the testcases:
[vbrandl/badgers@9c8cdb9](https://github.com/vbrandl/badgers/commit/9c8cdb91b81e95fb51f543237038cedeb2a3cb4d)

This fork exists mostly for personal use in
[`vbrandl/hoc`](https://github.com/vbrandl/hoc) but feel free to give it a try.
