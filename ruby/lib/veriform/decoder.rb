# frozen_string_literal: true

module Veriform
  # Build Veriform::Objects from Veriform's self-describing messages
  class Decoder
    # Create a new decoder object which will construct a Veriform::Object tree
    def initialize
      @stack = [Veriform::Object.new]
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

    # Push down the internal stack, constructing a new Veriform::Object
    def begin_nested
      @stack << Veriform::Object.new
    end

    # Complete the pushdown, adding the newly constructed object to the next one in the stack
    def end_nested(id)
      value = @stack.pop
      raise StateError, "not inside a nested message" if @stack.empty?
      @stack.last[id] = value
    end

    # Finish decoding, returning the parent Veriform::Object
    def finish
      result = @stack.pop
      raise StateError, "objects remaining in stack" unless @stack.empty?
      result
    end
  end
end
