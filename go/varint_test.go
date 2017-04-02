package zser

import (
	"bytes"
	"reflect"
	"testing"
)

func TestEncode(t *testing.T) {
	cases := []struct {
		num      uint64
		expected []byte
	}{
		{0, []byte("\x01")},
		{42, []byte("U")},
		{127, []byte("\xFF")},
		{128, []byte("\x02\x02")},
		{18446744073709551614, []byte("\x00\xFE\xFF\xFF\xFF\xFF\xFF\xFF\xFF")},
		{18446744073709551615, []byte("\x00\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF")},
	}

	for _, c := range cases {
		output := make([]byte, 9)
		length := EncodeVarint(output, c.num)

		if length != len(c.expected) {
			t.Errorf("EncodeVarint(%q) len: %q, want %q", c.num, length, len(c.expected))
		}

		if !reflect.DeepEqual(c.expected, output[:length]) {
			t.Errorf("EncodeVarint(%q) buf: %q, want %q", c.num, output[:length], c.expected)
		}
	}
}

func TestDecode(t *testing.T) {
	cases := []struct {
		input    []byte
		trailing []byte
		expected uint64
	}{
		// 0 with nothing trailing
		{[]byte("\x01"), []byte{}, 0},

		// 0 with trailing 0
		{[]byte("\x01\x00"), []byte{0}, 0},

		// 42 with trailing 0
		{[]byte("U\x00"), []byte{0}, 0},

		// 127 with trailing 0
		{[]byte("\xFF\x00"), []byte{0}, 0},

		// 128 with trailing 0
		{[]byte("\x02\x02\x00"), []byte{0}, 0},
	}

	for _, c := range cases {
		actual, _ := DecodeVarint(bytes.NewReader(c.input))
		if c.expected != actual {
			t.Errorf("DecodeVarint(%v) == %q, want %q", c.input, actual, c.expected)
		}
	}
}

func BenchmarkEncode(b *testing.B) {
	output := make([]byte, 9)

	for n := 0; n < b.N; n++ {
		EncodeVarint(output, 281474976741993)
	}
}

func BenchmarkDecode(b *testing.B) {
	input := []byte("\xE9\xF4\x81\x80\x80\x80@")

	for n := 0; n < b.N; n++ {
		DecodeVarint(bytes.NewReader(input))
	}
}
