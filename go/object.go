// object.go: Represents self-describing Veriform messages

package veriform

import (
	"fmt"
)

// Object is a deserialized Veriform message
type Object struct {
	Fields map[FieldID]interface{}
}

// NewObject creates a new Object instance
func NewObject() *Object {
	return &Object{make(map[FieldID]interface{})}
}

// LoadUint64 loads a uint64 value at the given field ID
func (o *Object) LoadUint64(fieldID FieldID) (uint64, error) {
	value, err := o.loadValue(fieldID)
	if err != nil {
		return 0, err
	}

	switch value := value.(type) {
	case uint64:
		return value, nil
	default:
		return 0, fmt.Errorf("field %d has type %T (expected uint64)", fieldID, value)
	}
}

// LoadBytes loads a []byte value at the given field ID
func (o *Object) LoadBytes(fieldID FieldID) ([]byte, error) {
	value, err := o.loadValue(fieldID)
	if err != nil {
		return nil, err
	}

	switch value := value.(type) {
	case []byte:
		return value, nil
	default:
		return nil, fmt.Errorf("field %d has type %T (expected []byte)", fieldID, value)
	}
}

// LoadObject loads a nested message object at the given field ID
func (o *Object) LoadObject(fieldID FieldID) (*Object, error) {
	value, err := o.loadValue(fieldID)
	if err != nil {
		return nil, err
	}

	switch value := value.(type) {
	case *Object:
		return value, nil
	default:
		return nil, fmt.Errorf("field %d has type %T (expected veriform.Object)", fieldID, value)
	}
}

// Store a value at the given field ID
func (o *Object) Store(fieldID FieldID, value interface{}) error {
	if _, ok := o.Fields[fieldID]; ok {
		return fmt.Errorf("duplicate field ID: %d", fieldID)
	}

	o.Fields[fieldID] = value

	return nil
}

// ToMap converts veriform.Objects into FieldID-indexed maps
func (o *Object) ToMap() map[FieldID]interface{} {
	result := make(map[FieldID]interface{})

	for fieldID, value := range o.Fields {
		switch value := value.(type) {
		case *Object:
			result[fieldID] = value.ToMap()
		default:
			result[fieldID] = value
		}
	}

	return result
}

// Retrieve a field by ID from the map, returning error if it's absent
func (o *Object) loadValue(fieldID FieldID) (interface{}, error) {
	value, ok := o.Fields[fieldID]

	if !ok {
		return nil, fmt.Errorf("message has no such field: %d", fieldID)
	}

	return value, nil
}
