package zser

import "testing"

func TestExample(t *testing.T) {
    if !Example() {
        t.Errorf("expected true, got false")
    }
}
