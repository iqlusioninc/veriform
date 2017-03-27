# encoding: utf-8
# frozen_string_literal: true

lib = File.expand_path("../lib", __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require "zser/version"

Gem::Specification.new do |spec|
  spec.name          = "zser"
  spec.version       = Zser::VERSION
  spec.authors       = ["Tony Arcieri"]
  spec.email         = ["bascule@gmail.com"]
  spec.summary       = "zcred serialization format"
  spec.description   = "A protobuf-inspired minimalistic serialization format with cryptographic authentication"
  spec.homepage      = "https://github.com/zcred/zser/tree/master/ruby/"
  spec.files         = `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
  spec.bindir        = "exe"
  spec.executables   = spec.files.grep(%r{^exe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.required_ruby_version = ">= 2.2.2"

  spec.add_development_dependency "bundler", "~> 1.14"
end
