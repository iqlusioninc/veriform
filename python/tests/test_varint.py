#!/usr/bin/env python

"""
test_varint
-----------

Tests for the zser `varint` module: prefixed variable-sized integers
"""

import unittest
from zser import varint
from zser.exceptions import TruncatedMessageError


class TestEncode(unittest.TestCase):
    def test_valid_examples(self):
        # 0
        self.assertEqual(varint.encode(0), b"\x01")

        # 42
        self.assertEqual(varint.encode(42), b"U")

        # 127
        self.assertEqual(varint.encode(127), b"\xFF")

        # 128
        self.assertEqual(varint.encode(128), b"\x02\x02")

        # 2**64-2
        self.assertEqual(varint.encode(18446744073709551614), b"\x00\xFE\xFF\xFF\xFF\xFF\xFF\xFF\xFF")

        # 2**64-1
        self.assertEqual(varint.encode(18446744073709551615), b"\x00\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF")

    def test_non_integer_param(self):
        with self.assertRaises(TypeError):
            varint.encode(0.5)

    def test_negative_param(self):
        with self.assertRaises(ValueError):
            varint.encode(-1)

    def test_oversize_param(self):
        with self.assertRaises(ValueError):
            varint.encode(varint.MAX + 1)


class TestDecode(unittest.TestCase):
    def test_valid_examples(self):
        # 0 with nothing trailing
        self.assertEqual(varint.decode(b"\x01"), (0, b""))

        # 0 with trailing 0
        self.assertEqual(varint.decode(b"\x01\0"), (0, b"\0"))

        # 42 with trailing 0
        self.assertEqual(varint.decode(b"U\0"), (42, b"\0"))

        # 127 with trailing 0
        self.assertEqual(varint.decode(b"\xFF\0"), (127, b"\0"))

        # 128 with trailing 0
        self.assertEqual(varint.decode(b"\x02\x02\0"), (128, b"\0"))

        # 2**64-2 with trailing 0
        self.assertEqual(varint.decode(b"\x00\xFE\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0"), (18446744073709551614, b"\0"))

        # 2**64-1 with trailing 0
        self.assertEqual(varint.decode(b"\x00\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0"), (18446744073709551615, b"\0"))

    def test_empty_input(self):
        with self.assertRaises(TruncatedMessageError):
            varint.decode(b"")

    def test_bad_type(self):
        with self.assertRaises(TypeError):
            varint.decode(42)


def test_encode_benchmark(benchmark):
    benchmark(varint.encode, 281474976741993)


def test_decode_benchmark(benchmark):
    benchmark(varint.decode, b"\xE9\xF4\x81\x80\x80\x80@")
