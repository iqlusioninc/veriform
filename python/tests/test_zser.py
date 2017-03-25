#!/usr/bin/env python

"""
test_zser
---------

Tests for the `zser` module.
"""

import unittest
import zser

class TestZser(unittest.TestCase):
    def test_decode(self):
        zser.decode("world!")
        pass
