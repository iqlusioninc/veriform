# frozen_string_literal: true

require "forwardable"

module Veriform
  # Key/value pairs ala JSON objects or Protobuf messages
  class Object
    extend Enumerable
    extend Forwardable

    # Delegate certain Hash functions to the underlying hash
    def_delegators :@fields, :each, :keys

    # Create a Veriform::Object from a TJSON::Object
    def self.from_tjson(obj)
      raise TypeError, "expected TJSON::Object, got #{obj.class}" unless obj.is_a?(TJSON::Object)

      new.tap do |result|
        obj.each do |key, value|
          result[Integer(key, 10)] = value.is_a?(TJSON::Object) ? from_tjson(value) : value
        end
      end
    end

    # Create a new Veriform::Object
    #
    # @return [Veriform::Object]
    def initialize
      @fields = {}
    end

    # Retrieve the value associated with a field identifier in a Veriform::Object
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
    # @raise [Veriform::DuplicateFieldError] attempt to set field that's already been set
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
        result[k] = v.is_a?(self.class) ? v.to_h : v
      end

      result
    end

    # Compare two Veriform::Objects by value for equality
    def eql?(other)
      return false unless other.is_a?(self.class)
      return false unless keys.length == other.keys.length

      keys.each do |key|
        return false unless self[key].eql?(other[key])
      end

      true
    end

    alias == eql?
  end
end
