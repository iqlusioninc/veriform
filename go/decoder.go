package veriform

// Decoder decodes self-describing Veriform to an Object representation
type Decoder struct {
	stack []*Object
}

// NewDecoder creates a new Decoder instance
func NewDecoder() *Decoder {
	return &Decoder{
		[]*Object{NewObject()},
	}
}

// Uint64 field callback
func (d *Decoder) Uint64(fieldID FieldID, value uint64) {
	if err := d.currentObject().Store(fieldID, value); err != nil {
		panic(err)
	}
}

// Bytes field callback
func (d *Decoder) Bytes(fieldID FieldID, data []byte) {
	if err := d.currentObject().Store(fieldID, data); err != nil {
		panic(err)
	}
}

// BeginNested signals we've entered a new nested message
func (d *Decoder) BeginNested() {
	d.stack = append(d.stack, NewObject())
}

// EndNested signals we've finished a nested message
func (d *Decoder) EndNested(fieldID FieldID) {
	if len(d.stack) == 0 {
		panic("not inside a nested message")
	}

	value := d.stack[len(d.stack)-1]
	d.stack = d.stack[:len(d.stack)-1]

	if err := d.currentObject().Store(fieldID, value); err != nil {
		panic(err)
	}
}

// Finish signals we've finished parsing
func (d *Decoder) Finish() interface{} {
	if len(d.stack) == 0 {
		panic("message stack is empty")
	} else if len(d.stack) > 1 {
		panic("messages remaining in stack")
	}

	return d.stack[0]
}

// Retrieve the current object on the stack
func (d *Decoder) currentObject() *Object {
	return d.stack[len(d.stack)-1]
}
