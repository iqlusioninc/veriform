package veriform

import (
	"encoding/binary"
	"errors"
	"io"
	"math/bits"
)

// EncodeVarint encodes a uint64 into buf and returns the number of bytes written.
// If the buffer is too small, EncodeVarint will panic.
func EncodeVarint(buf []byte, input uint64) int {
	length := 1
	value := (input << 1) | 1
	max := uint64(1 << 7)

	for input >= max {
		// 9-byte special case
		if length == 8 {
			buf[0] = 0
			binary.LittleEndian.PutUint64(buf[1:9], input)
			return 9
		}

		value <<= 1
		max <<= 7
		length++
	}

	var output [8]byte
	binary.LittleEndian.PutUint64(output[:], value)
	copy(buf, output[:length])
	return length
}

// DecodeVarint decodes a serialized vint64 into a uint64
func DecodeVarint(r io.Reader) (uint64, error) {
	var buf [8]byte

	_, err := r.Read(buf[:1])
	if err != nil {
		return 0, err
	}

	prefix := buf[0]
	if prefix == 0 {
		var result uint64
		err := binary.Read(r, binary.LittleEndian, &result)

		if err == nil && result < (1<<56) {
			return 0, errors.New("malformed varint")
		}

		return result, err
	}

	length := uint(bits.TrailingZeros8(prefix) + 1)

	_, err = io.ReadFull(r, buf[1:length])
	if err != nil {
		return 0, err
	}

	result := binary.LittleEndian.Uint64(buf[:])
	result >>= length
	if length > 1 && result < (1<<(7*(length-1))) {
		return 0, errors.New("malformed varint")
	}

	return result, nil
}
