# vint64 [![Crate][crate-image]][crate-link] [![Docs][docs-image]][docs-link] [![Apache 2.0 Licensed][license-image]][license-link] ![MSRV][msrv-image] [![Safety Dance][safety-image]][safety-link] [![Build Status][build-image]][build-link] 

`vint64` is an implementation of a variable-length encoding for 64-bit
little endian integers which optimizes for simplicity and performance.

[Documentation][docs-link]

## About

This crate implements a variable-length encoding for 64-bit little endian
integers with a number of properties which make it superior in almost every
way to other variable-length integer encodings like [LEB128], SQLite "Varuints",
or CBOR:

- Capable of expressing the full 64-bit integer range with a maximum of 9-bytes
- Total length of a `vint64` can be determined via the first byte alone
- Provides the most compact encoding possible for every value in range
- No loops involved in decoding: just (unaligned) loads, masks, and shifts
- No complex branch-heavy logic: decoding is CTZ + shifts and sanity checks

Some precedent for this sort of encoding can be found in the
[Extensible Binary Meta Language] (used by e.g. the [Matroska]
media container format), however note that the specific type of "vint"
used by that format still requires a loop to decode.

## License

Copyright © 2019-2020 Tony Arcieri

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be dual licensed as above,
without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/vint64.svg
[crate-link]: https://crates.io/crates/vint64
[docs-image]: https://docs.rs/vint64/badge.svg
[docs-link]: https://docs.rs/vint64/
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/veriform/blob/develop/LICENSE
[msrv-image]: https://img.shields.io/badge/rustc-1.40+-blue.svg
[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[safety-link]: https://github.com/rust-secure-code/safety-dance/
[build-image]: https://github.com/iqlusioninc/veriform/workflows/Rust/badge.svg?branch=develop&event=push
[build-link]: https://github.com/iqlusioninc/veriform/actions?query=workflow%3ARust

[//]: # (general links)

[LEB128]: https://en.wikipedia.org/wiki/LEB128
[Extensible Binary Meta Language]: https://en.wikipedia.org/wiki/Extensible_Binary_Meta_Language
[Matroska]: https://www.matroska.org/
