"""object.py: Represents self-describing Veriform messages"""

from collections import MutableMapping
from .exceptions import DuplicateFieldError


class Object(MutableMapping):
    """A dict-like type that represents a self-describing Veriform message"""

    def __init__(self, *args, **kwargs):
        self.fields = dict()
        self.update(dict(*args, **kwargs))

    def __getitem__(self, key):
        return self.fields[key]

    def __setitem__(self, key, value):
        if not isinstance(key, int):
            raise TypeError("key must be an integer: " + repr(key))

        if key < 0:
            raise ValueError("key must be positive: " + str(key))

        if key in self.fields:
            raise DuplicateFieldError("duplicate field ID: " + str(key))

        self.fields[key] = value

    def __delitem__(self, key):
        del self.fields[key]

    def __iter__(self):
        return iter(self.fields)

    def __len__(self):
        return len(self.fields)

    def __repr__(self):
        return "<veriform.object{0}>".format(repr(self.fields))

    def to_dict(self):
        result = dict()

        for (field_id, value) in self.fields.items():
            if isinstance(value, self.__class__):
                result[str(field_id)] = value.to_dict()
            else:
                result[str(field_id)] = value

        return result
