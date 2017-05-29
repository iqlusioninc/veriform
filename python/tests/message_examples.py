"""message_examples.py: Parse examples from vectors/messages.tjson"""

import binascii
import json
from collections import namedtuple

MessageExample = namedtuple("MessageExample", ["name", "description", "success", "encoded", "decoded"])

def load():
    return load_from_file("../vectors/messages.tjson")

def load_from_file(filename):
    examples_file = open(filename, "r")
    examples_text = examples_file.read()
    examples_file.close()

    examples_tjson = json.loads(examples_text)
    examples = examples_tjson[u"examples:A<O>"]

    result = []
    for example in examples:
        result.append(MessageExample(
            name=example[u"name:s"],
            description=example[u"description:s"],
            success=example[u"success:b"],
            encoded=binascii.unhexlify(example[u"encoded:d16"]),
            decoded=__decode_value(example.get(u"decoded:O"))
        ))

    return result

def __decode_value(input_dict):
    if input_dict == None:
        return None

    output_dict = dict()

    for (key, encoded_value) in input_dict.items():
        field_id, tag = key.split(":")

        if tag == "O":
            output_dict[field_id] = __decode_value(encoded_value)
        elif tag == "d16":
            output_dict[field_id] = binascii.unhexlify(encoded_value)
        elif tag == "u":
            output_dict[field_id] = int(encoded_value)

    return output_dict
