# frozen_string_literal: true

require "tjson"

class VarintExample
  attr_reader :value, :encoded, :success

  # Error parsing the example file
  ParseError = Class.new(StandardError)

  # Default file to load examples from
  DEFAULT_EXAMPLES = File.expand_path("../../../../vectors/varint.tjson", __FILE__)

  def self.load_file(filename = DEFAULT_EXAMPLES)
    examples = TJSON.load_file(filename).fetch("examples")
    raise ParseError, "expected a toplevel array of examples" unless examples.is_a?(Array)

    examples.map { |example| new(example) }
  end

  def initialize(attrs)
    @success = attrs.fetch("success")
    @value = attrs.fetch("value") if @success
    @encoded = attrs.fetch("encoded")
  end
end
