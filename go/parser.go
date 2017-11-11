// parser.go: veriform message parser

package veriform

import (
	"bytes"
	"fmt"
)

// Default maximum length of a veriform message: 1kB
// This is conservative as veriform's main intended use case is a credential format
const DEFAULT_MAX_LENGTH = 1024

// Default maximum depth (i.e. default max level of nested messages)
const DEFAULT_MAX_DEPTH = 8

// Parser for veriform messages
type parser struct {
	// Maximum length message we'll accept
	maxLength uint

	// Maximum depth of nested messages allowed
	maxDepth uint

	// Bodies of nested messages remaining to be processed
	remaining [][]byte

	// Callbacks to invoke to construct the resulting type
	callbacks handler
}

// Create a new Parser
func NewParser(callbacks handler) *parser {
	return &parser{
		DEFAULT_MAX_LENGTH,
		DEFAULT_MAX_DEPTH,
		make([][]byte, 0),
		callbacks,
	}
}

// Parse the given veriform message, invoking callbacks as necessary
func (p *parser) Parse(message []byte) error {
	if len(message) > int(p.maxLength) {
		return fmt.Errorf("oversized message: %d bytes (max %d)", len(message), p.maxLength)
	}

	if len(p.remaining) >= int(p.maxDepth) {
		return fmt.Errorf("max depth of %d nested messages exceeded", p.maxDepth)
	}

	p.remaining = append(p.remaining, message)

	for len(p.remaining[len(p.remaining)-1]) > 0 {
		fieldID, wireType, err := p.parseFieldPrefix()
		if err != nil {
			return err
		}

		switch wireType {
		case 0:
			err = p.parseUint64(fieldID)
		case 2:
			err = p.parseMessage(fieldID)
		case 3:
			err = p.parseBytes(fieldID)
		default:
			err = fmt.Errorf("unknown wiretype: %d", wireType)
		}

		if err != nil {
			return err
		}
	}

	p.remaining = p.remaining[:len(p.remaining)-1]

	return nil
}

// Finish parsing, returning the resulting object produced by the builder
func (p *parser) Finish() (interface{}, error) {
	if len(p.remaining) == 0 {
		return p.callbacks.Finish(), nil
	} else {
		return nil, fmt.Errorf("not finished parsing: %d messages in buffer", len(p.remaining))
	}
}

// Pop the top item in the remaining stack and parse a varint from it
// TODO: better integrate io.Reader to avoid unnecessary slicing
func (p *parser) parseVarint() (uint64, []byte, error) {
	slice := p.remaining[len(p.remaining)-1]
	reader := bytes.NewReader(slice)
	p.remaining = p.remaining[:len(p.remaining)-1]

	value, err := DecodeVarint(reader)
	if err != nil {
		return 0, nil, err
	}

	return value, slice[len(slice)-reader.Len():], nil
}

// Parse the integer each field starts with, extracting field ID and wiretype
func (p *parser) parseFieldPrefix() (FieldID, WireType, error) {
	value, remaining, err := p.parseVarint()
	if err != nil {
		return 0, 0, err
	}

	p.remaining = append(p.remaining, remaining)

	fieldID := FieldID(value >> 3)
	wireType := WireType(value & 0x7)

	return fieldID, wireType, nil
}

// Parse a u64 value stored as a prefix varint
func (p *parser) parseUint64(fieldID FieldID) error {
	value, remaining, err := p.parseVarint()
	if err != nil {
		return err
	}

	p.remaining = append(p.remaining, remaining)
	p.callbacks.Uint64(fieldID, value)

	return nil
}

// Parse a blob of data that begins with a length prefix
func (p *parser) parseLengthPrefixedData() ([]byte, error) {
	length, remaining, err := p.parseVarint()
	if err != nil {
		return nil, err
	}

	if len(remaining) < int(length) {
		return nil, fmt.Errorf("message truncated: want %d bytes (have %d)", length, len(remaining))
	}

	result := remaining[:length]
	p.remaining = append(p.remaining, remaining[length:])

	return result, nil
}

// Parse a nested message
func (p *parser) parseMessage(fieldID FieldID) error {
	p.callbacks.BeginNested()

	nestedMessage, err := p.parseLengthPrefixedData()
	if err != nil {
		return err
	}

	err = p.Parse(nestedMessage)
	if err != nil {
		return err
	}

	p.callbacks.EndNested(fieldID)
	return nil
}

// Parse a field containing binary data
func (p *parser) parseBytes(fieldID FieldID) error {
	data, err := p.parseLengthPrefixedData()
	if err != nil {
		return err
	}

	p.callbacks.Bytes(fieldID, data)
	return nil
}

// Callback API used by the parser to process parsed data
type handler interface {
	// Called when a uint64 value with the given field ID is parsed
	Uint64(fieldID FieldID, value uint64)

	// Called when we've received binary data with the given ID
	Bytes(fieldID FieldID, data []byte)

	// Indicate we've entered a new nested message
	BeginNested()

	// Indicate we've reached the end of a nested message with the given ID
	EndNested(fieldID FieldID)

	// Return the fully parsed object
	Finish() interface{}
}
