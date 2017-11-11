package veriform

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"io/ioutil"
	"reflect"
	"strconv"
	"testing"
)

type varintExample struct {
	Value   *uint64
	Encoded []byte
	Success bool
}

// Load common test examples from messages.tjson
// TODO: switch to a native Go TJSON parser when available
func loadVarintExamples() []varintExample {
	var examplesJSON map[string]interface{}

	exampleData, err := ioutil.ReadFile("../vectors/varint.tjson")
	if err != nil {
		panic(err)
	}

	if err = json.Unmarshal(exampleData, &examplesJSON); err != nil {
		panic(err)
	}

	examplesArray := examplesJSON["examples:A<O>"].([]interface{})

	if examplesArray == nil {
		panic("no toplevel 'examples:A<O>' key in varint.tjson")
	}

	result := make([]varintExample, len(examplesArray))

	for i, exampleJSON := range examplesArray {
		example := exampleJSON.(map[string]interface{})
		encodedHex := example["encoded:d16"].(string)
		encoded := make([]byte, hex.DecodedLen(len(encodedHex)))

		var value *uint64

		if _, ok := example["value:u"]; ok {
			v, err := strconv.ParseUint(example["value:u"].(string), 10, 64)
			if err != nil {
				panic(err)
			}

			value = &v
		}

		if _, err := hex.Decode(encoded, []byte(encodedHex)); err != nil {
			panic(err)
		}

		result[i] = varintExample{value, encoded, example["success:b"].(bool)}
	}

	return result
}

func TestEncodeVarint(t *testing.T) {
	for _, ex := range loadVarintExamples() {
		if !ex.Success {
			continue
		}

		output := make([]byte, 9)
		length := EncodeVarint(output, *ex.Value)

		if length != len(ex.Encoded) {
			t.Errorf("EncodeVarint(%q) len: %q, want %q", ex.Value, length, len(ex.Encoded))
		}

		if !reflect.DeepEqual(ex.Encoded, output[:length]) {
			t.Errorf("EncodeVarint(%q) buf: %q, want %q", ex.Value, output[:length], ex.Encoded)
		}
	}
}

func TestDecodeVarint(t *testing.T) {
	for _, ex := range loadVarintExamples() {
		actual, err := DecodeVarint(bytes.NewReader(ex.Encoded))

		if ex.Success {
			if *ex.Value != actual {
				t.Errorf("DecodeVarint(%v) == %v, want %v", ex.Encoded, actual, ex.Value)
			}
		} else if err == nil {
			t.Errorf("expected DecodeVarint(%v) to err but it succeeded", ex.Encoded)
		}
	}
}

func BenchmarkEncodeVarint(b *testing.B) {
	output := make([]byte, 9)

	for n := 0; n < b.N; n++ {
		EncodeVarint(output, 281474976741993)
	}
}

func BenchmarkDecodeVarint(b *testing.B) {
	input := []byte("\xE9\xF4\x81\x80\x80\x80@")

	for n := 0; n < b.N; n++ {
		if _, err := DecodeVarint(bytes.NewReader(input)); err != nil {
			panic(err)
		}
	}
}
