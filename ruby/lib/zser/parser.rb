# frozen_string_literal: true

module Zser
  # Parses encoded zser messages, invoking callbacks in the given handler
  # (i.e. this is a "push parser" which supports different backends)
  class Parser
    # Default maximum length of a zser message. This is a conservative choice
    # as zser's main intended use is a credential format.
    MAX_LENGTH = 1024

    # Default maximum depth (i.e. number of levels of child objects)
    MAX_DEPTH = 8

    # Create a new message parser with the given parse event handler
    def initialize(handler, max_length = MAX_LENGTH, max_depth = MAX_DEPTH)
      @handler = handler
      @max_length = max_length
      @max_depth = max_depth
      @remaining = []
    end

    # Parse the given zser message, invoking callbacks as necessary
    def parse(msg)
      raise OversizeMessageError, "length #{msg.length} exceeds max of #{@max_length}" if msg.length > @max_length
      raise EncodingError, "expected BINARY encoding, got #{msg.encoding}" unless msg.encoding == Encoding::BINARY
      @remaining << msg

      raise DepthError, "exceeded max depth of #{@max_depth}" if @remaining.size > @max_depth

      until @remaining.last.empty?
        id, wiretype = parse_field_prefix

        case wiretype
        when 0 then parse_uint64(id)
        when 2 then parse_message(id)
        when 3 then parse_binary(id)
        else raise ParseError, "unknown wiretype: #{wiretype.inspect}"
        end
      end

      @remaining.pop

      true
    end

    # Finish parsing, returning the resulting object produced by the builder
    def finish
      @handler.finish
    end

    private

    # Parse a varint which also stores a wiretype
    def parse_field_prefix
      result, remaining = Zser::Varint.decode(@remaining.pop)
      @remaining << remaining
      wiretype = result & 0x7
      [result >> 3, wiretype]
    end

    # Parse an unsigned 64-bit integer
    def parse_uint64(id)
      value, remaining = Zser::Varint.decode(@remaining.pop)
      @remaining << remaining
      @handler.uint64(id, value)
    end

    # Parse a data type stored with a length prefix
    def parse_length_prefixed_data
      length, remaining = Zser::Varint.decode(@remaining.pop)
      raise EOFError, "not enough bytes remaining in input" if remaining.bytesize < length
      data = remaining.byteslice(0, length)
      @remaining << remaining.byteslice(length, remaining.bytesize - length)
      data
    end

    # Parse a nested message
    def parse_message(id)
      @handler.begin_nested
      parse(parse_length_prefixed_data)
      @handler.end_nested(id)
    end

    # Parse length-prefixed binary data
    def parse_binary(id)
      @handler.binary(id, parse_length_prefixed_data)
    end
  end
end
