# frozen_string_literal: true

require "json"
require "toml"

class MessageExample
  attr_reader :name, :description, :success, :encoded, :decoded

  # Error parsing the example file
  ParseError = Class.new(StandardError)

  # Default file to load examples from
  DEFAULT_EXAMPLES = File.expand_path("../../../../examples/zser_messages.toml", __FILE__)

  def self.load_file(filename = DEFAULT_EXAMPLES)
    toml = TOML.load_file(filename)
    examples = toml["example"]
    raise ParseError, "expected a toplevel array of examples" unless examples.is_a?(Array)

    examples.map { |example| new(example) }
  end

  def initialize(attrs)
    @name = attrs.fetch("name")
    @description = attrs.fetch("description")
    @success = attrs.fetch("success")
    @encoded = attrs.fetch("encoded")

    decoded = attrs["decoded"]
    @decoded = JSON.parse(decoded) if decoded
  end

  def encoded_bytes
    [@encoded].pack("H*")
  end
end
