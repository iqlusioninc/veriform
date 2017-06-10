"""varint_examples.py: Parse examples from vectors/varint.tjson"""

import binascii
import json
from collections import namedtuple

VarintExample = namedtuple("VarintExample", ["value", "encoded", "success"])


def load():
    """Load varint examples from vectors/varint.tjson"""
    return load_from_file("../vectors/varint.tjson")


def load_from_file(filename):
    """Load varint examples from the specified file"""
    examples_file = open(filename, "r")
    examples_text = examples_file.read()
    examples_file.close()

    examples_tjson = json.loads(examples_text)
    examples = examples_tjson[u"examples:A<O>"]

    result = []
    for example in examples:
        value = None
        success = example[u"success:b"]
        if success:
            value = int(example[u"value:u"])

        result.append(VarintExample(
            value=value,
            encoded=binascii.unhexlify(example[u"encoded:d16"]),
            success=success
        ))

    return result
