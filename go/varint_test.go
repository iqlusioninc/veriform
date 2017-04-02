package zser

import (
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
		actual := EncodeVarint(c.num)
		if !reflect.DeepEqual(c.expected, actual) {
			t.Errorf("EncodeVarint(%q) == %q, want %q", c.num, actual, c.expected)
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
		actual, _ := DecodeVarint(c.input)
		if c.expected != actual {
			t.Errorf("DecodeVarint(%v) == %q, want %q", c.input, actual, c.expected)
		}
	}
}
