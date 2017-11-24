# encoding: binary
# frozen_string_literal: true

RSpec.describe Veriform::Verihash do
  context "verihash.tjson examples" do
    VerihashExample.load_file.each do |ex|
      it ex.name do
        value = ex.value.is_a?(TJSON::Object) ? Veriform::Object.from_tjson(ex.value) : ex.value
        expect(described_class.hexdigest(value)).to eql ex.digest.unpack("H*").first
      end
    end
  end

  describe ".hexdigest" do
    context "Integer" do
      it "raises RangeError if given a negative number" do
        expect { described_class.hexdigest(-1) }.to raise_error(RangeError)
      end

      it "raises RangeError if given an oversized number" do
        expect { described_class.hexdigest(18_446_744_073_709_551_616) }.to raise_error(RangeError)
      end
    end
  end
end
