#!/usr/bin/env python

"""
test_varint
-----------

Tests for the veriform `varint` module: prefixed variable-sized integers
"""

import unittest
from veriform import varint
from veriform.exceptions import TruncatedMessageError, ParseError

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