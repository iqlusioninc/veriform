"""varint.py: Little Endian 64-bit Unsigned Prefix Varints"""

from .exceptions import ParseError, TruncatedMessageError
import numbers
import struct
import sys

# Maximum value we can encode as an unsigned vint64
MAX = (2**64) - 1

# Lookup table for the number of trailing zeroes in a byte
CTZ_TABLE = [8, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
             4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
             5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
             4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
             6, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
             4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
             5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
             4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
             7, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
             4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
             5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
             4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
             6, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
             4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
             5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
             4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0]


def encode(value):
    """Encode the given integer value as an unsigned vint64"""
    if not isinstance(value, numbers.Integral):
        raise TypeError("value must be an integer")

    if value < 0:
        raise ValueError("value must be zero or greater")

    if value > MAX:
        raise ValueError("value must be in the 64-bit unsigned range")

    length = 1
    result = (value << 1) | 1
    max = 1 << 7

    while value >= max:
        # 9-byte special case
        if length == 8:
            return struct.pack("<BQ", 0, value)
        result <<= 1
        max <<= 7
        length += 1

    return struct.pack("<Q", result)[:length]


def decode(encoded):
    """Decode a vint64-serialized value into an unsigned integer"""
    if not isinstance(encoded, bytes):
        raise TypeError("input must be a byte array")

    if not encoded:
        raise TruncatedMessageError("not enough bytes to decode varint")

    if sys.version_info >= (3, 0):
        prefix = encoded[0]
    else:
        prefix = ord(encoded[0])

    if prefix == 0:
        length = 9
    else:
        length = CTZ_TABLE[prefix] + 1

    if len(encoded) < length:
        raise TruncatedMessageError("not enough bytes to decode varint")

    if prefix == 0:
        decoded = struct.unpack("<Q", encoded[1:9])[0]
    else:
        decoded = struct.unpack("<Q", encoded[:length] + b"\0" * (8 - length))[0] >> length

    if length > 1 and decoded < (1 << (7 * (length - 1))):
        raise ParseError("malformed varint")

    return decoded, encoded[length:]
