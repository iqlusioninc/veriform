# encoding: binary
# frozen_string_literal: true

module Zser
  # zsint: Little Endian 64-bit Unsigned Prefix Varints
  module Varint
    # Maximum value we can encode as a zsuint64
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

    # Encode the given integer value as a zsuint64
    #
    # @param value [Integer] unsigned integer value to encode as a zsuint64
    #
    # @raise [TypeError] non-integer value given
    # @raise [ArgumentError] value outside the unsigned 64-bit integer range
    #
    # @return [String] serialized zsuint64 value
    def self.encode(value)
      raise TypeError, "value must be an Integer" unless value.is_a?(Integer)
      raise ArgumentError, "value must be zero or greater" if value < 0
      raise ArgumentError, "value must be in the 64-bit unsigned range" if value > MAX

      length = 1
      result = (value << 1) | 1
      max = 1 << 7

      while value >= max
        # 9-byte special case
        return [0, value].pack("CQ<") if max == 1 << 63

        result <<= 1
        max <<= 7
        length += 1
      end

      [result].pack("Q<")[0, length].force_encoding(Encoding::BINARY)
    end

    # Decode a zsuint64-serialized value into an integer
    #
    # @param input [String] serialized zsuint64 to decode
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
        raise EOFError, "not enough bytes to decode varint" if input_len < 9
        [read_le64(input[1, 8]), input.byteslice(9, input_len - 9)]
      else
        # Count trailing zeroes
        count = CTZ_TABLE[prefix] + 1
        raise EOFError, "not enough bytes to decode varint" if input_len < count
        [read_le64(input[0, count]) >> count, input.byteslice(count, input_len - count)]
      end
    end

    class << self
      private

      # Decode a little endian integer (without allocating memory, unlike pack)
      def read_le64(bytes)
        result = 0

        (bytes.bytesize - 1).downto(0) do |i|
          result = (result << 8) | bytes.getbyte(i)
        end

        result
      end
    end
  end
end
