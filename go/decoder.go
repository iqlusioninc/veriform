package veriform

type decoder struct {
	stack []*Object
}

func NewDecoder() *decoder {
	return &decoder{
		[]*Object{NewObject()},
	}
}

// Called when a uint64 value with the given field ID is parsed
func (d *decoder) Uint64(fieldID FieldID, value uint64) {
	d.currentObject().Store(fieldID, value)
}

// Called when we've received binary data with the given ID
func (d *decoder) Bytes(fieldID FieldID, data []byte) {
	d.currentObject().Store(fieldID, data)
}

// Indicate we've entered a new nested message
func (d *decoder) BeginNested() {
	d.stack = append(d.stack, NewObject())
}

// Indicate we've reached the end of a nested message with the given ID
func (d *decoder) EndNested(fieldID FieldID) {
	if len(d.stack) == 0 {
		panic("not inside a nested message")
	}

	value := d.stack[len(d.stack)-1]
	d.stack = d.stack[:len(d.stack)-1]

	d.currentObject().Store(fieldID, value)
}

// Return the fully parsed object
func (d *decoder) Finish() interface{} {
	if len(d.stack) == 0 {
		panic("message stack is empty")
	} else if len(d.stack) > 1 {
		panic("messages remaining in stack")
	}

	return d.stack[0]
}

// Retrieve the current object on the stack
func (d *decoder) currentObject() *Object {
	return d.stack[len(d.stack)-1]
}
