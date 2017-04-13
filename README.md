# ![zser][zser-logo-image]

[![Build Status][build-image]][build-link]
[![MIT licensed][license-image]][license-link]

[zser-logo-image]: https://raw.githubusercontent.com/zcred/logos/master/zser-logo-md.png
[build-image]: https://secure.travis-ci.org/zcred/zser.svg?branch=master
[build-link]: http://travis-ci.org/zcred/zser
[license-image]: https://img.shields.io/badge/license-MIT-blue.svg
[license-link]: https://github.com/zcred/zser/blob/master/LICENSE.txt

**zser** (pronounced zē′zər like "Caesar") is a security-oriented serialization
format with novel authentication properties based on "Merkleized" data
structures.

## [Oh no!] Not another serialization format!

[Oh no!]: https://scottlocklin.wordpress.com/2017/04/02/please-stop-writing-new-serialization-protocols/

*[Obligatory xkcd](https://xkcd.com/927/)*

The decision to create a new serialization format was not undertaken lightly.
This format was created as a prerequisite of the [zcred] project, which
originally started as a Protobuf-based format ([protocreds]). Unfortunately
[protobuf limitations] made it difficult to implement zser's distinguishing
feature: Merkleized content authentication, while still supporting schema
evolution.

The wire representation of zser largely resembles protobufs, but with an
number of improvements including a self-describing structure, a UTF-8 inspired
prefix varint structure, and a richer set of wire types.

zser is not intended to be a general purpose serialization format: for that
we recommend [Cap'n Proto]. For example zser does not have an associated RPC
protocol, but rather zser is the sort of thing you might use for the
credentials passed as part of your RPC protocol.

zser's data model is isomorphic with [TJSON], a microformat for extending
JSON with richer types. All zser documents can be bidirectionally
transcoded to/from TJSON with no data loss. Furthermore, TJSON documents
can be authenticated with the same Merkleized hashing scheme as zser,
meaning signatures for one encoding will validate in the other.

[zcred]: https://github.com/zcred/zcred
[protocreds]: https://github.com/protocreds/
[protobuf limitations]: https://github.com/google/protobuf/issues/2629
[Cap'n Proto]: https://capnproto.org/
[TJSON]: https://www.tjson.org/

## Comparison with other serialization formats

The table below compares zser to the other formats considered
(and rejected) for the zcred use case:

| Name          | Schemas         | Self-Describing  | Integers        | Authentication   | Standardization |
|---------------|-----------------|------------------|-----------------|------------------|-----------------|
| [zser]        | :green_heart:†  | :green_heart:    | Prefix-Varint   | Merkleized†      | None            |
| [Protobuf]    | :green_heart:   | :broken_heart:   | [LEB128]        | Canonicalization | None            |
| [Cap'n Proto] | :green_heart:   | :green_heart:    | Fixed-Width     | Canonicalization | None            |
| [CBOR]        | :broken_heart:  | :green_heart:    | Fixed-Width     | Canonicalization | IETF            |
| [ASN.1 DER]   | :broken_heart:  | :yellow_heart:   | Fixed-Width     | Canonicalization | ITU/IETF        |
| [MessagePack] | :broken_heart:  | :green_heart:    | Fixed-Width     | None             | None            |

*†NOTE: Coming soon!*

[zser]: https://github.com/zcred/zser
[Protobuf]: https://developers.google.com/protocol-buffers/
[CBOR]: https://tools.ietf.org/html/rfc7049
[ASN.1 DER]: https://www.itu.int/rec/T-REC-X.690-201508-I/en
[MessagePack]: https://msgpack.org/
[LEB128]: https://en.wikipedia.org/wiki/LEB128

## Status

![DANGER: EXPERIMENTAL](https://raw.github.com/cryptosphere/cryptosphere/master/images/experimental.png)

zser does not yet provide the minimum viable functionality it needs to be
useful. The table below covers the current implementation state:

| Status             | Feature               | Notes             |
|--------------------|-----------------------|-------------------|
| :white_check_mark: | Prefix Varints        | Fully implemented |
| :construction:     | Message Decoding      | In progress       |
| :no_entry:         | Message Encoding      |                   |
| :no_entry:         | Merkle Authentication |                   |
| :no_entry:         | Schemas               |                   |
| :no_entry:         | [TJSON] Transcoding   |                   |

NOTE: zser is a multi-language monorepo: all implementations in all languages
within the repo are intended to implement the spec in its current state and
share a consistent feature set. The progress above applies equally to all
language implementations currently within the repo.

## Language Support

Packages implementing zser are available for the following languages:

| Language               | Version                              |
|------------------------|--------------------------------------|
| [Go][go-link]          | N/A                                  |
| [JavaScript][npm-link] | [![npm][npm-shield]][npm-link]       |
| [Python][pypi-link]    | [![pypi][pypi-shield]][pypi-link]    |
| [Ruby][gem-link]       | [![gem][gem-shield]][gem-link]       |
| [Rust][crate-link]     | [![crate][crate-shield]][crate-link] |


[go-link]: https://github.com/zcred/zser/tree/master/go
[npm-shield]: https://img.shields.io/npm/v/zser.svg
[npm-link]: https://www.npmjs.com/package/zser
[pypi-shield]: https://img.shields.io/pypi/v/zser.svg
[pypi-link]: https://pypi.python.org/pypi/zser/
[gem-shield]: https://badge.fury.io/rb/zser.svg
[gem-link]: https://rubygems.org/gems/zser
[crate-shield]: https://img.shields.io/crates/v/zser.svg
[crate-link]: https://crates.io/crates/zser

## Encoding

zser is a binary encoding format similar to [Google Protocol Buffers][Protobuf]
designed to have a compact, self-describing structure that supports
"Merkleized" content authentication using a git-like tree hashing algorithm.

### zsint: Prefix Varints with a 64-bit Range

Like [Protocol Buffers][Protobuf], zser uses variable-width integers to
provide a more compact encoding. Protocol Buffers use a format called
[LEB128], a Base128 encoding not unlike a 128-bit version of Base64.
This encoding uses the high bit to signal there are more bytes remaining,
and serializes values in little endian order. This format can serialize
integers of arbitrary size.

However, while LEB128 makes it easy to serialize numbers of any size,
it makes it harder to serialize numbers of a fixed width. There isn't
a natural way to cap LEB128 to 64-bits. A 64-bit number represented with
LEB128 requires 10 bytes.

An alternative approach to a variable-width integer encoding is employed
by formats like [UTF-8]. This format encodes the number of total bytes
into the prefix byte of the integer: in UTF-8's case, by the number of
leading `1` digits in the first byte.

zsints use a similar encoding: they include a count of the number of
bytes in the number using the *trailing zeroes* in the first byte:

| Prefix     | Precision | Total Bytes |
|------------|-----------|-------------|
| `xxxxxxx1` | 7 bits    | 1 byte      |
| `xxxxxx10` | 14 bits   | 2 bytes     |
| `xxxxx100` | 21 bits   | 3 bytes     |
| `xxxx1000` | 28 bits   | 4 bytes     |
| `xxx10000` | 35 bits   | 5 bytes     |
| `xx100000` | 42 bits   | 6 bytes     |
| `x1000000` | 49 bits   | 7 bytes     |
| `10000000` | 56 bits   | 8 bytes     |
| `00000000` | 64 bits   | 9 bytes     |

Using trailing zeroes means we can take advantage of CPU instructions such as
`CTZ` (count trailing zeroes) or `BSF` (bit scan forward) to count the number
of zeroes in the prefix byte.

This approach also provides a natural way to encode a 64-bit integer, since
8 * 8 = 64. Eight zeroes in the first byte (i.e. if the first byte is `0`)
indicates the next 8 bytes form a 64-bit little endian integer value,
allowing the full 64-bit range to be naturally expressed in 9-bytes at most.

Anything smaller is still encoded as little endian, but shifted over by the
number of bits in the length prefix. Once we've decoded the bytes as little
endian, all we have to do to finish decoding is a right bitwise shift to
remove the prefix bits.

This means the decoder can be implemented completely free of loops and
optimized implementations can decode varints in around *15 nanoseconds*.

[UTF-8]: https://en.wikipedia.org/wiki/UTF-8#Description

## Copyright

Copyright (c) 2017 [The Zcred Developers][AUTHORS].
See [LICENSE.txt] for further details.

[AUTHORS]: https://github.com/zcred/zcred/blob/master/AUTHORS.md
[LICENSE.txt]: https://github.com/zcred/zser/blob/master/LICENSE.txt
