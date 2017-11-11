"""exceptions.py: Custom exceptions used by Veriform"""


class ParseError(Exception):
    """Generic parse error"""


class TruncatedMessageError(ParseError):
    """Unexpected end of input"""


class OversizeMessageError(ParseError):
    """Message is larger than our maximum configured size"""


class DepthError(ParseError):
    """Nested message structure is too deep"""


class StateError(ParseError):
    """Parser is in the wrong state to perform the given task"""


class DuplicateFieldError(ParseError):
    """Field repeated in message"""
