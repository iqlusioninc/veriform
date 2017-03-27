# encoding: binary
# frozen_string_literal: true

module Zser
  # zsint: Little Endian 64-bit Unsigned Prefix Varints
  module Varint
    module_function

    # Maximum value we can encode as a zsuint64
    MAX = (2**64) - 1

    # Encode the given integer value as a zsuint64
    def encode(value)
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
    def decode(input)
      raise TypeError, "input must be a String" unless input.is_a?(String)
      raise ArgumentError, "input cannot be empty" if input.empty?
      prefix = input[0].ord

      # 9-byte special case
      return input[1, 8].unpack("Q<").first if prefix.zero?

      count = 1

      # Count trailing zeroes
      while (prefix & 1).zero?
        count += 1
        prefix >>= 1
      end

      (input + "\0" * (8 - input.length)).unpack("Q<")[0] >> count
    end
  end
end
