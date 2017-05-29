#!/usr/bin/env python

"""
test_parser
-----------

Tests for the zser `parser` module: parses binary-encoded zser messages
"""

import unittest

from zser.decoder import Decoder
from zser.exceptions import ParseError
from zser.parser import Parser

from . import message_examples

class TestParser(unittest.TestCase):
    def test_messages_tjson(self):
        for example in message_examples.load():
            parser = Parser(Decoder())

            if example.success:
                parser.parse(example.encoded)
                value = parser.finish()
                self.assertEqual(value.to_dict(), example.decoded)
            else:
                with self.assertRaises(ParseError):
                    parser.parse(example.encoded)
