package zser

import (
	"bytes"
	"encoding/binary"
	"errors"
)

func EncodeVarint(input uint64) []byte {
	output := new(bytes.Buffer)
	length := 1
	result := (input << 1) | 1
	max := uint64(1 << 7)

	for input >= max {
		// 9-byte special case
		if max == 1<<63 {
			output.WriteByte(0)
			binary.Write(output, binary.LittleEndian, input)
			return output.Bytes()
		}

		result <<= 1
		max <<= 7
		length += 1
	}

	binary.Write(output, binary.LittleEndian, result)
	return output.Bytes()[0:length]
}

func DecodeVarint(input []byte) (uint64, error) {
	var result uint64

	if len(input) == 0 {
		return 0, errors.New("cannot decode empty data")
	}

	prefix := input[0]

	if prefix == 0 {
		if len(input) >= 9 {
			buf := bytes.NewReader(input)
			binary.Read(buf, binary.LittleEndian, result)
			return result, nil
		} else {
			return 0, errors.New("not enough data remaining")
		}
	}

	count := uint(1)

	for prefix&1 == 0 {
		count += 1
		prefix >>= 1
	}

	if uint(len(input)) < count {
		return 0, errors.New("not enough data remaining")
	}

	slice := make([]byte, 8)
	copy(slice, input)
	buf := bytes.NewReader(slice)
	binary.Read(buf, binary.LittleEndian, result)

	return result >> count, nil
}
