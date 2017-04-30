# frozen_string_literal: true

require "tjson"

class MessageExample
  attr_reader :name, :description, :success, :encoded, :decoded

  # Error parsing the example file
  ParseError = Class.new(StandardError)

  # Default file to load examples from
  DEFAULT_EXAMPLES = File.expand_path("../../../../vectors/messages.tjson", __FILE__)

  def self.load_file(filename = DEFAULT_EXAMPLES)
    examples = TJSON.load_file(filename).fetch("examples")
    raise ParseError, "expected a toplevel array of examples" unless examples.is_a?(Array)

    examples.map { |example| new(example) }
  end

  def initialize(attrs)
    @name = attrs.fetch("name")
    @description = attrs.fetch("description")
    @success = attrs.fetch("success")
    @encoded = attrs.fetch("encoded")
    @decoded = attrs.fetch("decoded") if @success
  end
end
