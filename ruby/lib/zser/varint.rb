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

      [result].pack("Q<")[0, length]
    end

    # Decode a zsuint64-serialized value into an integer
    def self.decode(input)
      raise TypeError, "input must be a String" unless input.is_a?(String)
      raise ArgumentError, "input cannot be empty" if input.empty?

      prefix = input.getbyte(0)

      if prefix.zero?
        # 9-byte special case
        read_le64(input[1, 8])
      else
        # Count trailing zeroes
        count = CTZ_TABLE[prefix] + 1
        read_le64(input[0, count]) >> count
      end
    end

    # Decode a little endian integer (without allocating memory, unlike pack)
    def self.read_le64(bytes)
      result = 0

      (bytes.length - 1).downto(0) do |i|
        result = (result << 8) | bytes.getbyte(i)
      end

      result
    end
  end
end
