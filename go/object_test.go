package veriform

import (
	"reflect"
	"testing"
)

const uintField = FieldID(1)
const byteField = FieldID(2)
const objField = FieldID(3)
const invalidField = FieldID(4)

func exampleUint() uint64 {
	return 42
}

func exampleBytes() []byte {
	return []byte("Hello, world!")
}

func nestedObject() *Object {
	return NewObject()
}

func exampleObject() *Object {
	obj := NewObject()
	obj.Store(uintField, exampleUint())
	obj.Store(byteField, exampleBytes())
	obj.Store(objField, nestedObject())
	return obj
}

func TestLoadUint64(t *testing.T) {
	obj := exampleObject()

	// Test actual uint64 value
	actual, err := obj.LoadUint64(uintField)
	if err != nil {
		t.Errorf("LoadUint64(uintField) returned error: %v", err)
		return
	}

	if exampleUint() != actual {
		t.Errorf("LoadUint64(uintField) == %v, want %v", actual, exampleUint())
	}

	// Test non-uint64 value
	_, err = obj.LoadUint64(byteField)
	if err == nil {
		t.Error("LoadUint64(byteField) didn't return error")
	}

	// Test missing value value
	_, err = obj.LoadUint64(invalidField)
	if err == nil {
		t.Error("LoadUint64(invalidField) didn't return error")
	}
}

func TestLoadBytes(t *testing.T) {
	obj := exampleObject()

	// Test actual []byte value
	actual, err := obj.LoadBytes(byteField)
	if err != nil {
		t.Errorf("LoadBytes(byteField) returned error: %v", err)
		return
	}

	if !reflect.DeepEqual(exampleBytes(), actual) {
		t.Errorf("LoadBytes(byteField) == %v, want %v", actual, exampleBytes())
	}

	// Test non-bytes value
	_, err = obj.LoadBytes(uintField)
	if err == nil {
		t.Error("LoadBytes(uintField) didn't return error")
	}

	// Test missing value value
	_, err = obj.LoadBytes(invalidField)
	if err == nil {
		t.Error("LoadBytes(invalidField) didn't return error")
	}
}

func TestLoadObject(t *testing.T) {
	obj := exampleObject()

	// Test actual []byte value
	actual, err := obj.LoadObject(objField)
	if err != nil {
		t.Errorf("LoadObject(objField) returned error: %v", err)
		return
	}

	if !reflect.DeepEqual(nestedObject(), actual) {
		t.Errorf("LoadObject(objField) == %v, want %v", actual, nestedObject())
	}

	// Test non-object value
	_, err = obj.LoadObject(uintField)
	if err == nil {
		t.Error("LoadObject(uintField) didn't return error")
	}

	// Test missing value value
	_, err = obj.LoadObject(invalidField)
	if err == nil {
		t.Error("LoadObject(invalidField) didn't return error")
	}
}
