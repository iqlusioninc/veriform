%%%

    Title = "Zcred Serialization (Zser) Data Interchange Format"
    abbrev = "Zser Data Interchange Format"
    category = "info"
    docName = "draft-zser-spec"

    date = 2017-03-26

    [[author]]
    initials = "T. "
    surname = "Arcieri"
    fullname = "Tony Arcieri"

%%%

.# Abstract

The Zcred Serialization Format (zser) is a binary encoding for structured data
designed to support operating with or without schemas, supporting a compact
on-wire representation, and with advanced cryptographic authentication features
similar to Merkle trees. The format is inspired by Google's Protocol Buffers,
but makes a number of changes aimed at improving simplicity, performance, and
security.

{mainmatter}

# Introduction

The Zcred Serialization Format (zser) was designed in service of serializing
a credential format, and as such makes many decisions aimed at maximizing
the format's security properties, as opposed to being a general purpose
serialization format.

One such decision is reducing the types which can be expressed by the format.
In zser, some common types such as floats and signed integers are omitted.
This keeps the format simple and parsimonious for security use cases.

zser supports five scalar types:

* Strings
* Binary Data
* Integers (unsigned only)
* Timestamps
* Booleans

It supports two non-scalar types:

* Objects
* Arrays

## Conventions Used in This Document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in [@!RFC2119].

# Wire Encoding

This section describes the on-the-wire representations of the various types
provided by zser.

## Prefix Varints

Integers in zser are variable-width, allowing a highly compact representation
of small integer values. However, unlike Protocol Buffers, zser does not use
Little Endian Base 128 (LEB128) encoding, but instead places all continuation
bits at the beginning of the serialized value, ala UTF-8 [@!RFC3629].

Below is an example of how prefix bits signal the length of the integer value
which follows:

    7-bits:  0xxxxxxx
    14-bits: 10xxxxxx xxxxxxxx
    21-bits: 110xxxxx xxxxxxxx xxxxxxxx
    ...
    64-bits: 11111111 110xxxxx xxxxxxxx ... xxxxxxxx 00000xxx

The bits within a varint should be interpreted as a little endian integer of
the given size, representing values in the range `[0, (2**64)−1]`.
