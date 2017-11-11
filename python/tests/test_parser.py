#!/usr/bin/env python

"""
test_parser
-----------

Tests for the veriform `parser` module: parses binary-encoded Veriform messages
"""

import unittest

from veriform.decoder import Decoder
from veriform.exceptions import ParseError
from veriform.parser import Parser

from .support import message_examples

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
