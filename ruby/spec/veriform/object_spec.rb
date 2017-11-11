# frozen_string_literal: true

RSpec.describe Veriform::Object do
  subject(:object) { described_class.new }

  describe "#[]=" do
    it "allows Integer keys" do
      object[1] = 42
    end

    it "raises TypeError on non-Integer keys" do
      expect { object["hello"] = 42 }.to raise_error TypeError
    end

    it "raises RangeError on negative keys" do
      expect { object[-1] = 42 }.to raise_error RangeError
    end

    it "raises Veriform::DuplicateFieldError if key is set twice" do
      object[1] = 42
      expect { object[1] = 42 }.to raise_error Veriform::DuplicateFieldError
    end
  end
end
