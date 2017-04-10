# frozen_string_literal: true

module Zser
  # Parses encoded zser messages, invoking callbacks in the given handler
  # (i.e. this is a "push parser" which supports different backends)
  class Parser
    # Create a new message parser with the given parse event handler
    def initialize(handler)
      @handler = handler
      @remaining = nil
    end

    # Parse the given zser message, invoking callbacks as necessary
    def parse(msg)
      raise StateError, "already parsing a message" if @remaining
      raise EncodingError, "expected BINARY encoding, got #{msg.encoding}" unless msg.encoding == Encoding::BINARY

      @remaining = msg
      until @remaining.empty?
        id, wiretype = parse_field_prefix

        case wiretype
        when 0 then parse_uint64(id)
        when 2 then parse_message(id)
        when 3 then parse_binary(id)
        else raise ParseError, "unknown wiretype: #{wiretype.inspect}"
        end
      end

      true
    end

    # Finish parsing, returning the resulting object produced by the builder
    def finish
      @handler.finish
    end

    private

    # Parse a varint which also stores a wiretype
    def parse_field_prefix
      result, @remaining = Zser::Varint.decode(@remaining)
      wiretype = result & 0x7
      [result >> 3, wiretype]
    end

    # Parse a length prefix, and ensure enough data is remaining in the buffer
    def parse_length_prefix
      length, @remaining = Zser::Varint.decode(@remaining)
      raise EOFError, "not enough bytes remaining in input" if @remaining.bytesize < length
      length
    end

    # Parse an unsigned integer
    def parse_uint64(id)
      value, @remaining = Zser::Varint.decode(@remaining)
      @handler.uint64(id, value)
    end

    # Parse a nested message
    def parse_message(id)
      length = parse_length_prefix

      @handler.begin_nested
      self.class.new(@handler).parse(@remaining.byteslice(0, length))
      @handler.end_nested(id)

      @remaining = @remaining.byteslice(length, @remaining.bytesize - length)
    end

    # Parse length-prefixed binary data
    def parse_binary(id)
      length = parse_length_prefix
      @handler.binary(id, @remaining.byteslice(0, length))
      @remaining = @remaining.byteslice(length, @remaining.bytesize - length)
    end
  end
end
