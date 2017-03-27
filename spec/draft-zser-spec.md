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

## Little Endian Prefix Varints (zsint)

Integers in zser are variable-width, allowing a highly compact representation
of small integer values. However, unlike Protocol Buffers, zser does not use
Little Endian Base 128 (LEB128) encoding, but instead places all continuation
bits at the beginning of the serialized value, ala UTF-8 [@!RFC3629]. Though
this format has been independently reinvented several times, this document
will describe it as "zsint" to distinguish this particular construction.

Little endian byte order has been controversial when used in IETF standards
(see e.g. [@!IEN137]), but is the predominant standard used by the
overwhelming majority of computing devices in the world. For that reason
we choose to embrace the encoding, rather than "network byte order"
(i.e. big endian).

### 64-bit Unsigned zsints (zsuint64)

`zsuint64` is presently the only encoding of zsints supported.

Integers serialized as `zsuint64` are (up to) 64-bit unsigned little endian
integers, which supports the full `[0, (2**64)−1]` unsigned 64-bit integer
range. They have serialized lengths from 1-byte to 9-bytes depending on
what value they're representing. The number of remaining bytes is stored
in the leading byte, indicated by the number of trailing zeroes in that byte.

Below is an example of how prefix bits signal the length of the integer value
which follows:

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

All arithmetic needed to serialize and deserialize `zsuint64` can be performed
using only 64-bit integers. The case of the prefix byte being all-zero is
a special case, and any remaining arithmetic is performed on the remaining
bytes.
