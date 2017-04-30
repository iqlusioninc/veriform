# frozen_string_literal: true

module Zser
  # Base class of all Zser errors
  Error = Class.new(StandardError)

  # Generic parse error
  ParseError = Class.new(Error)

  # Data is not in the correct character encoding
  EncodingError = Class.new(ParseError)

  # Unexpected end of input
  EOFError = Class.new(ParseError)

  # Message is larger than our maximum configured size
  OversizeMessageError = Class.new(ParseError)

  # Nested message structure is too deep
  DepthError = Class.new(ParseError)

  # Parser is in the wrong state to perform the given task
  StateError = Class.new(ParseError)

  # Field repeated in message
  DuplicateFieldError = Class.new(ParseError)
end
