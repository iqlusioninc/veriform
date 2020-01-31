# ![Veriform](https://raw.githubusercontent.com/clasp-lang/veriform/develop/img/veriform.png)

[![Build Status][build-image]][build-link]
[![MIT licensed][license-image]][license-link]

**Veriform** is a cryptographically verifiable data serialization format
inspired by [Protocol Buffers], useful for things like credentials, transparency
logs, and "blockchain" applications.

[Specification]

## Rationale (a.k.a. "[Oh no! Not another serialization format!][ohno]")

> If you look at another engineer's work and think, "That's dumb. Why don't you just..."
> Take a breath. Find out why the problem is hard. _—[Adrienne Porter Felt][apf]_

*[Obligatory xkcd]*

This format was created as a prerequisite of the [zcred] project, which
originally started as a Protobuf-based format ([protocreds]). Unfortunately
[protobuf limitations] made it difficult to implement Veriform's distinguishing
feature: Merkleized content authentication, while still supporting schema
evolution.

The wire representation of Veriform largely resembles Protobufs, but with an
number of improvements including a self-describing structure based on a
richer set of wire types. The format is simple to implement despite its
improved expressiveness, provides a simple and limited degree of compression
with variable-sized integers (using a UTF-8 inspired prefix varint structure),
and is designed to be free of integer operations that may potentially overflow.

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

The table below compares Veriform to the other formats considered
(and rejected) for the zcred use case:

| Name          | Schemas         | Self-Describing  | Integers        | Authentication     | Standardization |
|---------------|-----------------|------------------|-----------------|--------------------|-----------------|
| [Veriform]    | :green_heart:†  | :green_heart:    | Prefix-Varint   | Structured Hashing | None            |
| [Protobuf]    | :green_heart:   | :broken_heart:   | [LEB128]        | Canonicalization   | None            |
| [Cap'n Proto] | :green_heart:   | :green_heart:    | Fixed-Width     | Canonicalization   | None            |
| [CBOR]        | :broken_heart:  | :green_heart:    | Fixed-Width     | Canonicalization   | IETF            |
| [ASN.1 DER]   | :broken_heart:  | :yellow_heart:   | Fixed-Width     | Canonicalization   | ITU/IETF        |
| [csexp]       | :broken_heart:  | :green_heart:    | Fixed-Width     | Canonicalization   | IETF            |
| [MessagePack] | :broken_heart:  | :green_heart:    | Fixed-Width     | None               | None            |

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
See [LICENSE.txt] for further details.

[//]: # (badges)

[build-image]: https://github.com/clasp-lang/veriform/workflows/Rust/badge.svg?branch=develop&event=push
[build-link]: https://github.com/clasp-lang/veriform/actions
[license-image]: https://img.shields.io/badge/license-MIT-blue.svg
[license-link]: https://github.com/clasp-lang/veriform/blob/develop/LICENSE.txt

[//]: # (general links)

[Specification]: https://github.com/clasp-lang/veriform/blob/develop/spec/draft-veriform-spec.md
[ohno]: https://scottlocklin.wordpress.com/2017/04/02/please-stop-writing-new-serialization-protocols/
[apf]: https://twitter.com/__apf__/status/867751153026482177
[Obligatory xkcd]: https://xkcd.com/927/
[Protocol Buffers]: https://developers.google.com/protocol-buffers/
[zcred]: https://github.com/zcred/zcred
[protocreds]: https://github.com/protocreds/
[protobuf limitations]: https://github.com/google/protobuf/issues/2629
[Cap'n Proto]: https://capnproto.org/
[TJSON]: https://www.tjson.org/
[LICENSE.txt]: https://github.com/clasp-lang/veriform/blob/develop/LICENSE.txt

[//]: # (comparison table links)

[Veriform]: https://github.com/zcred/veriform
[Protobuf]: https://developers.google.com/protocol-buffers/
[CBOR]: https://tools.ietf.org/html/rfc7049
[ASN.1 DER]: https://www.itu.int/rec/T-REC-X.690-201508-I/en
[MessagePack]: https://msgpack.org/
[csexp]: https://en.wikipedia.org/wiki/Canonical_S-expressions
[LEB128]: https://en.wikipedia.org/wiki/LEB128
