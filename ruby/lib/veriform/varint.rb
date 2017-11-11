# encoding: binary
# frozen_string_literal: true

module Veriform
  # Little Endian 64-bit Unsigned Prefix Varints
  module Varint
    # Maximum value we can encode as a vint64
    MAX = (2**64) - 1

    # :nodoc: Lookup table for the number of trailing zeroes in a byte
    CTZ_TABLE = [8, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
                 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
                 5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
                 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
                 6, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
                 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
                 5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
                 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
                 7, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
                 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
                 5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
                 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
                 6, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
                 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
                 5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
                 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0].freeze

    # Encode the given unsignedinteger value as a vint64
    #
    # @param value [Integer] unsigned integer value to encode as a vint64
    #
    # @raise [TypeError] non-integer value given
    # @raise [ArgumentError] value outside the unsigned 64-bit integer range
    #
    # @return [String] serialized vint64 value
    def self.encode(value)
      raise TypeError, "value must be an Integer" unless value.is_a?(Integer)
      raise ArgumentError, "value must be zero or greater" if value < 0
      raise ArgumentError, "value must be in the 64-bit unsigned range" if value > MAX

      length = 1
      result = (value << 1) | 1
      max = 1 << 7

      while value >= max
        # 9-byte special case
        return [0, value].pack("CQ<") if length == 8

        result <<= 1
        max <<= 7
        length += 1
      end

      [result].pack("Q<")[0, length].force_encoding(Encoding::BINARY)
    end

    # Decode a vint64-serialized value into an unsignedinteger
    #
    # @param input [String] serialized vint64 to decode
    #
    # @raise [TypeError] non-String input given
    # @raise [ArgumentError] empty input given
    #
    # @return [Array<Integer, String>] decoded integer and remaining data
    def self.decode(input)
      raise TypeError, "input must be a String" unless input.is_a?(String)
      raise ArgumentError, "input cannot be empty" if input.empty?

      prefix = input.getbyte(0)
      input_len = input.bytesize

      # 9-byte special case
      if prefix.zero?
        raise TruncatedMessageError, "not enough bytes to decode varint" if input_len < 9
        length = 9
        result = decode_le64(input[1, 8])
      else
        # Count trailing zeroes
        length = CTZ_TABLE[prefix] + 1
        result = decode_le64(input[0, length]) >> length
        raise TruncatedMessageError, "not enough bytes to decode varint" if input_len < length
      end

      if length > 1 && result < (1 << (7 * (length - 1)))
        raise ParseError, "malformed varint"
      end

      [result, input.byteslice(length, input_len - length)]
    end

    class << self
      private

      # Decode a little endian integer (without allocating memory, unlike pack)
      def decode_le64(bytes)
        result = 0

        (bytes.bytesize - 1).downto(0) do |i|
          result = (result << 8) | bytes.getbyte(i)
        end

        result
      end
    end
  end
end
