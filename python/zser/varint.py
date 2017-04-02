import numbers
import struct

# Maximum value we can encode as a zsuint64
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
    """Encode the given integer value as a zsuint64"""
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
        if max == 1 << 63:
            return struct.pack("<BQ", 0, value)
        result <<= 1
        max <<= 7
        length += 1

    return struct.pack("<Q", result)[:length]

def decode(input):
    """Decode a zsuint64-serialized value into an integer"""
    if not isinstance(input, bytes):
        raise TypeError("input must be a byte array")

    if not input:
        raise ValueError("input cannot be empty")

    prefix = input[0]

    if not isinstance(prefix, int):
        prefix = ord(prefix)

    if prefix == 0:
        return struct.unpack("<Q", input[1:9])[0]
    else:
        count = CTZ_TABLE[prefix] + 1
        return struct.unpack("<Q", input + b"\0" * (8 - len(input)))[0] >> count
