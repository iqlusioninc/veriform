"""parser.py: Parse encoded Veriform messages"""

from . import varint
from .exceptions import *

#: Default maximum length of a Veriform message.
MAX_LENGTH = 1024

#: Default maximum depth (i.e. number of levels of child objects)
MAX_DEPTH = 8


class Parser(object):
    """Parses encoded Veriform messages, invoking callbacks in the given handler
    (i.e. this is a "push parser" which supports different backends)"""

    def __init__(self, handler, max_length = MAX_LENGTH, max_depth = MAX_DEPTH):
        """Create a new message parser with the given parse event handler"""
        self.handler = handler
        self.max_length = max_length
        self.max_depth = max_depth
        self.remaining = []

    def parse(self, msg):
        """Parse the given Veriform message, invoking callbacks as necessary"""
        if not isinstance(msg, bytes):
            raise TypeError("msg must be bytes")

        if len(msg) > self.max_length:
            raise OversizeMessageError("length {0} exceeds max of {1}".format(len(msg), self.max_length))

        self.remaining.append(msg)

        if len(self.remaining) > self.max_depth:
            raise DepthError("max depth of {0} nested messages exceeded".format(self.max_depth))

        while self.remaining[-1]:
            field_id, wiretype = self.__parse_field_prefix()

            if wiretype == 0:
                self.__parse_uint64(field_id)
            elif wiretype == 2:
                self.__parse_message(field_id)
            elif wiretype == 3:
                self.__parse_binary(field_id)
            else:
                raise ParseError("unknown wiretype: " + repr(wiretype))

        return self.remaining.pop()

    def finish(self):
        """Finish parsing, returning the resulting object produced by the builder"""
        return self.handler.finish()

    def __parse_field_prefix(self):
        """Parse a varint which also stores a wiretype"""
        result, remaining = varint.decode(self.remaining.pop())
        self.remaining.append(remaining)
        wiretype = result & 0x7

        return result >> 3, wiretype

    def __parse_uint64(self, field_id):
        """Parse an unsigned 64-bit integer"""
        value, remaining = varint.decode(self.remaining.pop())
        self.remaining.append(remaining)
        self.handler.uint64(field_id, value)

    def __parse_length_prefixed_data(self):
        """Parse a data type stored with a length prefix"""
        length, remaining = varint.decode(self.remaining.pop())

        if len(remaining) < length:
            raise TruncatedMessageError("not enough bytes remaining in input")

        data = remaining[:length]
        self.remaining.append(remaining[length:])

        return data

    def __parse_message(self, field_id):
        """Parse a nested message"""
        self.handler.begin_nested()
        self.parse(self.__parse_length_prefixed_data())
        self.handler.end_nested(field_id)

    def __parse_binary(self, field_id):
        """Parse length-prefixed binary data"""
        self.handler.binary(field_id, self.__parse_length_prefixed_data())
