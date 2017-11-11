# frozen_string_literal: true

require "digest"

module Veriform
  # Computes astructured hash of a Veriform message
  class Zhash
    # One character "tag" values used to separate zhash domains
    module Tags
      # "Objects" represent Veriform messages
      OBJECT = "O"

      # 8-bit clean binary data
      BINARY = "d"

      # 64-bit unsigned integers
      UINT64 = "u"
    end

    # By default we compute zhashes using SHA-256
    DEFAULT_HASH_ALGORITHM = Digest::SHA256

    # Calculate the zhash digest of the given object
    #
    # @param algorithm [Class] a class which behaves like a `Digest`
    #
    # @return [String] bytestring containing the resulting digest
    def self.digest(object, algorithm: DEFAULT_HASH_ALGORITHM)
      new(algorithm: algorithm).digest(object)
    end

    # Calculate an object's zhash digest, hex encoding the result.
    # Takes the same parameters as `Veriform::Zhash.digest`
    #
    # @return [String] hex encoded string containing the resulting digest
    def self.hexdigest(object, **args)
      digest(object, **args).unpack("H*").first
    end

    # Create a new `Zhash` instance
    #
    # @param algorithm [Class] a class which behaves like a `Digest` (i.e. implements `reset`, `update`, `digest`)
    #
    # @return [Veriform::Zhash]
    def initialize(algorithm: DEFAULT_HASH_ALGORITHM)
      @algorithm = algorithm
      @hasher = algorithm.new
    end

    # Compute the zhash of any object allowed in a Veriform message
    #
    # @param object [Veriform::Object, String, Integer] object to compute a Zhash from
    #
    # @return [String] bytestring containing the resulting digest
    def digest(object)
      case object
      when Veriform::Object, Hash then object_digest(object)
      when String then binary_digest(object)
      when Integer then uint64_digest(object)
      else raise TypeError, "can't compute zhash of #{object.class}"
      end
    end

    # Calculate an object's zhash digest, hex encoding the result.
    # Takes the same parameters as `Veriform::Zhash#digest`
    #
    # @return [String] hex encoded string containing the resulting digest
    def hexdigest(object)
      digest(object).unpack("H*").first
    end

    private

    # Compute digest of a `Veriform::Object`
    def object_digest(message)
      hasher = @algorithm.new
      hasher.update Tags::OBJECT

      message.keys.sort.each do |key|
        hasher.update(encode_uint64(key))
        hasher.update(digest(message[key]))
      end

      hasher.digest
    end

    # Compute digest of a bytestring
    def binary_digest(bytes)
      raise EncodingError, "expected BINARY encoding, got #{value.encoding}" unless bytes.encoding == Encoding::BINARY
      compute_tagged_digest(Tags::BINARY, bytes)
    end

    # Compute digest of an `Integer`
    def uint64_digest(value)
      compute_tagged_digest(Tags::UINT64, encode_uint64(value))
    end

    # Compute a hash of the given bytes, tweaking the first byte of the
    # resulting digest with the given domain separator tag.
    def compute_tagged_digest(tag, bytes)
      raise ArgumentError, "tag must be 1-byte" if tag.bytesize != 1
      @hasher.reset
      @hasher.update(tag)
      @hasher.update(bytes)
      @hasher.digest
    end

    # Encode a uin64 value as little endian
    #
    # @param value [Integer] a positive, up-to-64-bit integer value
    #
    # @return [String] a bytestring containing a little endian-encoded value
    def encode_uint64(value)
      raise TypeError, "expected Integer, got #{value.class}" unless value.is_a?(Integer)
      raise RangeError, "integer value must be positive" if value < 0
      raise RangeError, "integer value exceeds 2**64-1" if value > 18_446_744_073_709_551_615
      [value].pack("Q<")
    end
  end
end
