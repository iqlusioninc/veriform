# ![Veriform](https://raw.githubusercontent.com/clasp-lang/veriform/develop/img/veriform.png)

[![Build Status][build-image]][build-link]
[![Apache 2.0 licensed][license-image]][license-link]

**Veriform** is a cryptographically verifiable data serialization format
inspired by [Protocol Buffers], useful for things like credentials, transparency
logs, and "blockchain" applications.

[Specification]

## Rationale (a.k.a. "[Oh no! Not another serialization format!][ohno]")

> If you look at another engineer's work and think, "That's dumb. Why don't you just..."
> Take a breath. Find out why the problem is hard. _—[Adrienne Porter Felt][apf]_

*[Obligatory xkcd]*

Veriform is similar in design to [Protocol Buffers], which is at its core
a schema-driven format. It also includes a small amount of information which
makes it partly self-describing, "wire types", however they are too limited to
[comprehend serialized messages in absence of a schema][protobuf limitations].

Without a fully self-describing wire format, it isn't possible to implement
Veriform's primary distinguishing feature: "Merkleized" content hashing
which functions even if some of the fields in a message aren't known by the
schema. This means that the sender and receiver of a message don't need to
be working with the same version of a schema to agree on a message hash,
which enables schema evolution.

Veriform is not intended to be a general purpose serialization format: for that
we recommend something like [Protocol Buffers] or [Cap'n Proto]. While it's fine
to use Veriform for general purpose serialization if it fits your needs, it's
lacking many features such as an associated RPC protocol. Instead, Veriform is
the sort of thing you might use for the credentials passed as part of an RPC
protocol.

Another interesting use case for Veriform is Certificate Transparency or otherwise
"blockchain"-like systems that heavily rely on cryptographic integrity and
Merkle proofs.

Veriform's data model is isomorphic with a subset of [TJSON], a microformat
which extends JSON with richer types. All Veriform documents can be bidirectionally
transcoded to/from TJSON with no data loss. Furthermore, TJSON documents
can be authenticated with the same Merkleized hashing scheme as Veriform,
meaning signatures for one encoding will validate in the other.

## Comparison with other serialization formats

The table below compares Veriform to the other formats:

| Name          | Schemas         | Self-Describing  | Integers        | Authentication     | Standardization |
|---------------|-----------------|------------------|-----------------|--------------------|-----------------|
| **Veriform**  | :green_heart:†  | :green_heart:    | [vint64]        | Structured Hashing | None            |
| [ASN.1 DER]   | :broken_heart:  | :yellow_heart:   | Fixed-Width     | Canonicalization   | ITU/IETF        |
| [Cap'n Proto] | :green_heart:   | :green_heart:    | Fixed-Width     | Canonicalization   | None            |
| [CBOR]        | :broken_heart:  | :green_heart:    | Fixed-Width     | Canonicalization   | IETF            |
| [csexp]       | :broken_heart:  | :green_heart:    | Fixed-Width     | Canonicalization   | IETF            |
| [MessagePack] | :broken_heart:  | :green_heart:    | Fixed-Width     | None               | None            |
| [Protobuf]    | :green_heart:   | :broken_heart:   | [LEB128]        | Canonicalization   | None            |
| [XDR]         | :green_heart:   | :broken_heart:   | Fixed-Width     | None               | IETF            |

*†NOTE: Coming soon!*

## Status

Veriform does not yet provide the minimum viable functionality it needs to be
useful. The table below covers the current implementation state:

| Status             | Feature               | Notes             |
|--------------------|-----------------------|-------------------|
| :construction:     | Message Decoding      | In progress       |
| :no_entry:         | Message Encoding      | Not started       |
| :construction:     | Structured Hashing    | In progress       |
| :no_entry:         | Schemas               | Not started       |
| :construction:     | [TJSON] Transcoding   | In progress       |

NOTE: Veriform is a multi-language monorepo: all implementations in all languages
within the repo are intended to implement the spec in its current state and
share a consistent feature set. The progress above applies equally to all
language implementations currently within the repo.

## Copyright

Copyright © 2017-2020 Tony Arcieri

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
submitted for inclusion in the work by you shall be licensed as above,
without any additional terms or conditions.

[//]: # (badges)

[build-image]: https://github.com/clasp-lang/veriform/workflows/Rust/badge.svg?branch=develop&event=push
[build-link]: https://github.com/clasp-lang/veriform/actions
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/clasp-lang/veriform/blob/develop/LICENSE

[//]: # (general links)

[Specification]: https://github.com/clasp-lang/veriform/blob/develop/spec/draft-veriform-spec.md
[ohno]: https://scottlocklin.wordpress.com/2017/04/02/please-stop-writing-new-serialization-protocols/
[apf]: https://twitter.com/__apf__/status/867751153026482177
[Obligatory xkcd]: https://xkcd.com/927/
[Protocol Buffers]: https://developers.google.com/protocol-buffers/
[protobuf limitations]: https://github.com/google/protobuf/issues/2629
[TJSON]: https://www.tjson.org/
[vint64]: https://github.com/clasp-lang/veriform/blob/develop/spec/draft-veriform-spec.md#64-bit-unsigned-variable-width-integers-vint64

[//]: # (comparison table links)

[ASN.1 DER]: https://www.itu.int/rec/T-REC-X.690-201508-I/en
[Cap'n Proto]: https://capnproto.org/
[CBOR]: https://tools.ietf.org/html/rfc7049
[csexp]: https://en.wikipedia.org/wiki/Canonical_S-expressions
[MessagePack]: https://msgpack.org/
[LEB128]: https://en.wikipedia.org/wiki/LEB128
[Protobuf]: https://developers.google.com/protocol-buffers/
[XDR]: https://en.wikipedia.org/wiki/External_Data_Representation
