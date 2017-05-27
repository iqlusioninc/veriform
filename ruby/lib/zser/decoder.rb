# frozen_string_literal: true

module Zser
  # Build Zser::Objects from zser's self-describing form
  class Decoder
    # Create a new decoder object which will construct a Zser::Object tree
    def initialize
      @stack = [Zser::Object.new]
    end

    # Add a uint64 to the current object
    def uint64(id, value)
      raise TypeError, "expected Integer, got #{value.class}" unless value.is_a?(Integer)
      @stack.last[id] = value
    end

    # Add binary data to the current object
    def binary(id, value)
      raise TypeError, "expected String, got #{value.class}" unless value.is_a?(String)
      raise EncodingError, "expected BINARY encoding, got #{value.encoding}" unless value.encoding == Encoding::BINARY
      @stack.last[id] = value
    end

    # Push down the internal stack, constructing a new Zser::Object
    def begin_nested
      @stack << Zser::Object.new
    end

    # Complete the pushdown, adding the newly constructed object to the next one in the stack
    def end_nested(id)
      value = @stack.pop
      raise StateError, "not inside a nested message" if @stack.empty?
      @stack.last[id] = value
    end

    # Finish decoding, returning the parent Zser::Object
    def finish
      result = @stack.pop
      raise StateError, "objects remaining in stack" unless @stack.empty?
      result
    end
  end
end
