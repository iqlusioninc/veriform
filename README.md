# ![zser][zser-logo-image]

[![Build Status][build-image]][build-link]
[![MIT licensed][license-image]][license-link]

[zser-logo-image]: https://raw.githubusercontent.com/zcred/logos/master/zser-logo-md.png
[build-image]: https://secure.travis-ci.org/zcred/zser.svg?branch=master
[build-link]: http://travis-ci.org/zcred/zser
[license-image]: https://img.shields.io/badge/license-MIT-blue.svg
[license-link]: https://github.com/zcred/zser/blob/master/LICENSE.txt

A security-oriented serialization format with novel authentication properties.

## Oh no! Not another serialization format!

Surely by now there's not a need to create a new serialization format,
painstakingly describing its properties in minute detail and tediously
implementing it in every programming language under the sun. Aren't
Protobufs good enough?

The answer is no. The original work on zcreds began using a
[protobuf based format]. Unfortunately, one of the new features added
in zser (Merkleized content authentication) does not play nicely with
Protobufs due to a [limitation in the protobufs design].

However, that does not mean there is anything inherently wrong with these
other serialization formats. The intent of zser is *not* to design a
Protobuf-killer. If that's what you're looking for, we recommend you check out
[Cap'n Proto], an advanced, general-purpose serialization format which
corrects many design mistakes in Protobufs.

The zser format is specifically designed for compactly representing richly
structured, cryptographically authenticated data, with a flexible credential
format as the primary intended use case.

For security purposes, zser is much more constrainted than a richer format
like [Cap'n Proto]. Instead, zser is designed to be relatively simple,
schema-friendly yet still self-describing, well-typed, and designed in service
of an advanced and highly expressive "format independent" content
authentication scheme.

[protobuf based format]: https://github.com/protocreds/
[limitation in the protobufs design]: https://github.com/google/protobuf/issues/2629
[Cap'n Proto]: https://capnproto.org/

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

## Copyright

Copyright (c) 2017 [The Zcred Developers][AUTHORS].
See [LICENSE.txt] for further details.

[AUTHORS]: https://github.com/zcred/zcred/blob/master/AUTHORS.md
[LICENSE.txt]: https://github.com/zcred/zser/blob/master/LICENSE.txt
