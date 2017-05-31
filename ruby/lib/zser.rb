# frozen_string_literal: true

require "zser/version"
require "zser/exceptions"

require "zser/decoder"
require "zser/object"
require "zser/parser"
require "zser/varint"

# zcred serialization format
module Zser
  # Parse the given self-describing zser message
  #
  # @param message [String] binary encoded zser message
  #
  # @return [Zser::Object] Hash-like object representing message
  def self.parse(message)
    parser = Zser::Parser.new(Zser::Decoder.new)
    parser.parse(message)
    parser.finish
  end
end
