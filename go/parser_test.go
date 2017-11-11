package veriform

import (
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"reflect"
	"strconv"
	"strings"
	"testing"
)

type messageExample struct {
	Name        string
	Description string
	Success     bool
	Encoded     []byte
	Decoded     map[FieldID]interface{}
}

// Load common test examples from messages.tjson
// TODO: switch to a native Go TJSON parser when available
func loadMessageExamples() []messageExample {
	var examplesJson map[string]interface{}

	exampleData, err := ioutil.ReadFile("../vectors/messages.tjson")
	if err != nil {
		panic(err)
	}

	if err = json.Unmarshal(exampleData, &examplesJson); err != nil {
		panic(err)
	}

	examplesArray := examplesJson["examples:A<O>"].([]interface{})

	if examplesArray == nil {
		panic("no toplevel 'examples:A<O>' key in messages.tjson")
	}

	result := make([]messageExample, len(examplesArray))

	for i, exampleJson := range examplesArray {
		example := exampleJson.(map[string]interface{})
		encodedHex := example["encoded:d16"].(string)
		encoded := make([]byte, hex.DecodedLen(len(encodedHex)))

		if _, err := hex.Decode(encoded, []byte(encodedHex)); err != nil {
			panic(err)
		}

		decodedEntry := example["decoded:O"]
		decoded := make(map[FieldID]interface{})

		if decodedEntry != nil {
			decoded = decodeValue(decodedEntry.(map[string]interface{}))
		}

		result[i] = messageExample{
			example["name:s"].(string),
			example["description:s"].(string),
			example["success:b"].(bool),
			encoded,
			decoded,
		}
	}

	return result
}

func decodeValue(value map[string]interface{}) map[FieldID]interface{} {
	result := make(map[FieldID]interface{})

	for key, encodedValue := range value {
		keyParts := strings.Split(key, ":")

		fieldID, err := strconv.ParseUint(keyParts[0], 10, 64)
		if err != nil {
			panic(err)
		}

		tag := keyParts[1]

		if tag == "O" {
			result[FieldID(fieldID)] = decodeValue(encodedValue.(map[string]interface{}))
		} else if tag == "d16" {
			hexString := encodedValue.(string)
			bytes := make([]byte, hex.DecodedLen(len(hexString)))

			if _, err := hex.Decode(bytes, []byte(hexString)); err != nil {
				panic(err)
			}

			result[FieldID(fieldID)] = bytes
		} else if tag == "u" {
			value, err := strconv.ParseUint(encodedValue.(string), 10, 64)
			if err != nil {
				panic(err)
			}

			result[FieldID(fieldID)] = value
		} else {
			panic(fmt.Errorf("unknown tag: %v", tag))
		}
	}

	return result
}

func TestParsingMessageExamples(t *testing.T) {
	examples := loadMessageExamples()

	for _, example := range examples {
		parser := NewParser(NewDecoder())

		if example.Success {
			if err := parser.Parse(example.Encoded); err != nil {
				panic(err)
			}

			obj, err := parser.Finish()
			if err != nil {
				panic(err)
			}

			actual := obj.(*Object).ToMap()
			if !reflect.DeepEqual(actual, example.Decoded) {
				t.Errorf("expected: %+v, actual: %+v", example.Decoded, actual)
			}
		} else {
			if err := parser.Parse(example.Encoded); err == nil {
				t.Errorf("expected example '%s' to error but it succeeded", example.Name)
			}
		}
	}
}
