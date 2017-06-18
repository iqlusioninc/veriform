# frozen_string_literal: true

require "bundler/setup"
require "zser"
require "support/message_examples"
require "support/varint_examples"
require "support/zhash_examples"

RSpec.configure(&:disable_monkey_patching!)
