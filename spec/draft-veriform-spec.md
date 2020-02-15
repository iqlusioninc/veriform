%%%

    Title = "The Veriform Data Interchange Format"
    abbrev = "Veriform"
    category = "info"
    docName = "draft-veriform-spec"

    date = 2020-02-15

    [[author]]
    initials = "T. "
    surname = "Arcieri"
    fullname = "Tony Arcieri"

%%%

.# Abstract

The Veriform Data Interchange Format is a binary encoding for structured data
designed to support operating with or without schemas, supporting a compact
on-wire representation, and cryptographically verifiable using a structured
content hashing algorithm reminiscent of (and providing similar properties to)
Merkle trees.

{mainmatter}

# Introduction

Veriform is aimed at maximizing the format's security properties, as opposed
to being a flexible data warehousing format like Protobufs.

It supports five scalar types:

* Unicode Strings (always encoded as UTF-8)
* Binary Data
* Integers (signed/unsigned)
* Timestamps
* Booleans

It supports two non-scalar types:

* Objects
* Arrays

## Conventions Used in This Document

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in [@!RFC2119].

# Encoding

This section describes the on-the-wire representations of the various types
provided by Veriform.

## Little Endian Prefixed Variable-Width Integers

Integers in Veriform are variable-width, allowing a highly compact representation
of small integer values. However, unlike Protocol Buffers, Veriform does not use
Little Endian Base 128 (LEB128) encoding, but instead places all continuation
bits at the beginning of the serialized value, ala UTF-8 [@!RFC3629]. Though
this format has been independently reinvented several times, this document
will describe it as "vint64" to distinguish this particular construction.

Little endian byte order has been controversial when used in IETF standards
(see e.g. [@!IEN137]), but is the predominant standard used by the
overwhelming majority of computing devices in the world. For that reason
we choose to embrace the encoding, rather than "network byte order"
(i.e. big endian).

### 64-bit Unsigned Variable-Width Integers (vint64)

Integers serialized as unsigned `vint64` are (up to) 64-bit unsigned little
endian integers, which supports the full `[0, (2**64)−1]` unsigned 64-bit
integer range. They have serialized lengths from 1-byte to 9-bytes depending on
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

All arithmetic needed to serialize and deserialize `vint64` can be performed
using only 64-bit integers. The case of the prefix byte being all-zero is
a special case, and any remaining arithmetic is performed on the remaining
bytes.

## Message Format

The Veriform message format builds upon vint64 integers, using them to encode
key/value pairs (called message entries) where the key is a vint64 and the
value is one of a set of different wire types with different encoding
properties.

Every entry in a message begins with a vint64 which encodes both an integer
field identifier (the "key" of the key/value pair) along with a wire type
identifier. The lower 3 bits of this initial varint contain the wire type
value, so the entire integer is encoded as follows:

    (field_number << 3) | wire_type

The following wire types are supported by Veriform:

| ID | Type                    | Encoding              |
|----|-------------------------|-----------------------|
| 0  | Unsigned 64-bit integer | vint64                |
| 1  | Signed 64-bit integer   | vint64 (zigzag)       |
| 2  | Nested Message          | Length Prefixed       |
| 3  | Binary Data             | Length Prefixed       |
| 4  | Unicode String          | Length Prefixed UTF-8 |
| 5  | Reserved                | N/A                   |
| 6  | Reserved                | N/A                   |
| 7  | Reserved                | N/A                   |

The "Length Prefixed" encoding consists of a single vint64 which indicates
the number of bytes in the subsequent value, followed by the value.

Field IDs MUST be unique. Any message containing repeated field IDs MUST be
rejected by compliant parsers.

# Structured Content Hashing (Verihash)

The Verihash algorithm computes a unique content hash for every field and nested
message present in a Veriform message. This approach to hashing can either happen
in tandem with deserialization, or by walking the object graph produced by
parsing a message. Only the content of messages is included in the hash
calculation, and not the serialized structure, allowing compatible hashes to
be computed for the same message serialized in other formats (e.g. TJSON).

By computing a hash based on the content of a message and not its literal
on-wire serialization, we avoid having to specify complex canonicalization
rules which ensure there is a singular "one true serialization" for each
message. Instead, specific field ordering is not required, and messages
serialized in different orders will produce the same content hash.

This approach to hashing is somewhat reminiscent of a Merkle tree. Like a
Merkle tree, structured hashing facilitates the ability to compute so-called
"inclusion proofs" for sub-messages, by providing a sub-message and the
necessary message digests of the surrounding structure to show that a
sub-message is rooted in a given parent hash.

Both Veriform and the Verihash algorithm have been designed specifically to support
authentication of content independent from the schema. This is the main
impetus for the format to be self-describing, and allows message generators
and verifiers with differing views of the schema to compute the same content
hash. This is necessary to support schema evolution while ensuring messages
containing unknown fields can still be cryptographically authenticated.

While Veriform's primary intended use case is for highly structured credentials,
it is assumed this structured hashing approach is applicable to other areas
that involve hashing of structured data, including so-called "blockchain" and
"transparency log" systems, particularly because these systems benefit highly
from support for "inclusion proofs".

## Acknowledgements

The design of the Verihash algorithm is heavily inspired by Ben Laurie's
[ObjectHash] algorithm, and is also a major impetus for the existence of this
entire project in the first place.

[ObjectHash]: https://github.com/benlaurie/objecthash

## Supported Hash Algorithms

This format is designed with the mindset that diversity of cryptographic
primitives is expensive and failure to provide normative advice about algorithm
selection has lead to complicated, incompatible, fractured ecosystems
containing a massive proliferation of algorithms. As such, all allowed hash
functions are explicitly specified in this document, and conforming
implementations MUST NOT implement any algorithms besides the ones
specifically enumerated below.

Each algorithm is identified by an all-upper-case alphanumeric short code
of variable length. These short codes SHOULD be used by all implementations
as the name of the constant or enum variant identifying these algorithms.

Conforming implementations MUST support selectable hash functions, and MUST NOT
be implemented in such a way that a single hash function is hard coded into
the implementation in an unselectable/inextensible manner.

### "SHA256"

The United States of America (USA) Federal Information Processing Standard
(FIPS) Secure Hash Algorithms (SHAs) specifies a suite of hash functions
with varying digest and block sizes.

Of these, SHA-256, as described in [RFC6234], is supported as a hash function
for use with Verihash. This function is mandatory to implement for all confirming
Verihash implementations, i.e. conforming implementations MUST implement SHA-256.

The string `SHA256` identifies this algorithm uniquely for the purposes of
Verihash. Conforming implementations should support selecting this hash function
using this string.

SHA-256 was selected as the primary hash function for use with Verihash for the
following reasons:

* The SHA-2 algorithm family is ubiquitous with fast software and hardware
  implementations available for all platforms where Veriform is applicable.
* The SHA-2 algorithm family has not been subject to a practical or
  theoretical attack, despite the initial results which lead to the SHA-3
  competition (which were inapplicable to SHA-2).
* SHA-256 in particular has been selected over alternatives such as SHA-512
  and SHA-512/256 despite software implementations of the 512-bit versions
  of these functions performing nearly twice as well on 64-bit architectures.
  This is because this standard is considering the performance on
  low-power/embedded devices with less-than-64-bit CPUs, where software
  SHA-256 will greatly outperform a software SHA-512 implementation. These
  devices increasingly support hardware SHA-256, much more than they support
  hardware SHA-512.
* In addition to better embedded performance, modern x86 CPUs across multiple
  vendors will support (or will soon support) the Intel SHA Extensions,
  providing a hardware accelerated implementation of SHA-256. The SHA-512
  algorithm is NOT supported by the SHA Extensions.

#### Security Considerations

The SHA-2 family is known to be vulnerable to length extension attacks due
to their use of the Merkle-Damgaard construction, which (after computing a
finalizer block) outputs its full internal state such that it can be used as a
"chaining variable".

However, individual fields within Verihash messages are not vulnerable to these
attacks, due to the use of structured hashing. As Verihash repeatedly hashes
digests of individual messages in order to compute a final hash for a message
object, repeat hashing prevents the extension of individual fields.

It should still be noted that it's possible to extend the toplevel message
with additional fields. No specific mitigation for this potential attack is
presently provided. When using SHA-256 as a hash function with Verihash, message
verifiers should be aware that the toplevel message is potentially extensible.

To completely avoid this class of attacks, a hash algorithm which is not
vulnerable to length extension attacks is recommended.

## Tagged Hashing

All Veriform wire types are given a unique tag which acts as a domain separation
mechanism. Scalars tags are lower-case, and non-scalar tags are upper case.

Domain separation provides important security properties: naive Merkle Trees
are vulnerable to second-preimage attacks in which an attacker convinces
a victim to hash a string, only for that string to be structurally significant
in the context of the Merkle Tree.

To avoid these attacks, we encode the type of hashed data into the digest
itself. This is accomplished by prepending the message to be hashed with a
tag character which uniquely identifies the data type. For example, to
compute a digest of the bytestring "Hello, world!" tagged with the character
"x" (not actually a valid tag, just an example), we can use the following
Python code:

    import hashlib

    tag, data = b"X", b"Hello, world!"
    hash = hashlib.sha256()
    hash.update(tag)
    hash.update(data)
    digest = hash.digest()

By tagging digests with their type, we ensure that each type produces a unique
digest, even if the serialized data is identical. This means that e.g. nested
messages can be readily disambiguated by a nested message serialized as binary
data, even though the wire format is otherwise identical (although the wire type
will differ).

## Data Types

The following section describes the hashing schemes used for the various data
types provided by Veriform.

### uint64 ("u")

To compute the digest of an unsigned integer, compute a tagged hash beginning
with the "u" character followed by the 64-bit little endian serialization of
a number.

For example, the hexadecimal representation of the SHA-256 digest of the
integer value "42" is:

    afed9cfd89625380e2ea8eb8bdd293d2c8149283b1ae2f5bd5a55ee8d9a8f27a

### Binary Data ("d")

To compute the digest of binary data, compute a tagged hash beginning with the
"d" character followed by the binary data to be hashed.

For example, the hexadecimal representation of the SHA-256 digest of the
ASCII string "Hello, world!" is:

    6ff091b89c1bdf783df27de366e1616f5d2f89ca46588c79f8c152b1fa5d698f

### Message Objects ("O")

Message objects are one of the non-scalar types provided by Veriform, and are
identified by a tag consisting of the upper case letter "O" (i.e. the letter,
NOT the digit zero)

To compute the hash of a message object, begin by sorting the IDs of each of
the fields. This is unambiguous as field IDs MUST be unique.

Begin computing the digest by hashing the "O", followed by the 64-bit little
endian serialization of the field ID. Next, compute Verihash for the value of
the field, and include that in the message digest, i.e.:

    "O" || Field #1 ID || Field #1 Hash || ... || Field #N ID || Field #N Hash

For example, the hexadecimal representation of the SHA-256 digest of a message
with a single field with an ID of "1" whose value is the ASCII string
"Hello, world!" serialied as binary data is:

    be0e50a6723c484b45aeaefa853337ecd161ab5fc613667b3dcd73f69d187ff8
