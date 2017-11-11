"""decoder.py: Decode Veriform's self-describing structure"""

from .exceptions import StateError
from .object import Object


class Decoder(object):
    """Decoder for Veriform's self-describing structure"""

    def __init__(self):
        """Create a new decoder object which will construct a Veriform object tree"""
        self.stack = [Object()]

    def uint64(self, field_id, value):
        """Add a uint64 to the current object"""
        if not isinstance(value, int):
            raise TypeError("expected Integer, got " + str(type(value)))

        self.stack[-1][field_id] = value

    def binary(self, field_id, value):
        """Add binary data to the current object"""
        if not isinstance(value, bytes):
            raise TypeError("expected bytes, got " + str(type(value)))

        self.stack[-1][field_id] = value

    def begin_nested(self):
        """Push down the internal stack, constructing a new Object"""
        self.stack.append(Object())

    def end_nested(self, field_id):
        """Complete the pushdown, adding the newly constructed object to the next one in the stack"""
        value = self.stack.pop()

        if not self.stack:
            raise StateError("not inside a nested message")

        self.stack[-1][field_id] = value

    def finish(self):
        """Finish decoding, returning the parent Object"""
        result = self.stack.pop()
        if self.stack:
            raise StateError("objects remaining in stack")

        return result
