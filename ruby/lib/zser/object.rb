# frozen_string_literal: true

module Zser
  # Key/value pairs ala JSON objects or Protobuf messages
  class Object
    # Create a new Zser::Object
    #
    # @return [Zser::Object]
    def initialize
      @fields = {}
    end

    # Retrieve the value associated with a field identifier in a Zser::Object
    #
    # @param key [Integer] field identifier
    #
    # @return [Object] value associated with this key
    def [](key)
      @fields[key]
    end

    # Sets the value associated with a field identifier
    #
    # @param key [Integer] field identifier
    # @param value [Object] value associated with the given key
    #
    # @raise [TypeError] non-Integer key given
    # @raise [Zser::DuplicateFieldError] attempt to set field that's already been set
    #
    # @return [Object] newly set value
    def []=(key, value)
      raise TypeError, "key must be an integer: #{key.inspect}" unless key.is_a?(Integer)
      raise RangeError, "key must be positive: #{key.inspect}" if key < 0
      raise DuplicateFieldError, "duplicate field ID: #{key}" if @fields.key?(key)

      @fields[key] = value
    end

    # Return a hash representation of this object (and its children).
    # This is akin to an `#as_json` method as seen in e.g. Rails.
    #
    # @return [Hash] a hash representation of this object
    def to_h
      result = {}

      @fields.each do |k, v|
        result[k.to_s] = v.is_a?(self.class) ? v.to_h : v
      end

      result
    end
  end
end
