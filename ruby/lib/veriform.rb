# frozen_string_literal: true

require "veriform/version"
require "veriform/exceptions"

require "veriform/decoder"
require "veriform/object"
require "veriform/parser"
require "veriform/varint"
require "veriform/zhash"

# Cryptographically verifiable data serialization format inspired by Protocol Buffers
module Veriform
  # Parse the given self-describing Veriform message
  #
  # @param message [String] binary encoded Veriform message
  #
  # @return [Veriform::Object] `::Hash`-like object representing message
  def self.parse(message)
    parser = Veriform::Parser.new(Veriform::Decoder.new)
    parser.parse(message)
    parser.finish
  end
end
