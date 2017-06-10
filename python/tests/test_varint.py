#!/usr/bin/env python

"""
test_varint
-----------

Tests for the zser `varint` module: prefixed variable-sized integers
"""

import unittest
from zser import varint
from zser.exceptions import TruncatedMessageError, ParseError

from .support import varint_examples

class TestEncode(unittest.TestCase):
    def test_valid_examples(self):
        for example in varint_examples.load():
            if example.success:
                self.assertEqual(varint.encode(example.value), example.encoded)

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
        for example in varint_examples.load():
            if example.success:
                self.assertEqual(varint.decode(example.encoded), (example.value, b""))
            else:
                with self.assertRaises(ParseError):
                    varint.decode(example.encoded)

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
