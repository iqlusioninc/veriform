# encoding: binary
# frozen_string_literal: true

RSpec.describe Zser::Varint do
  describe ".encode" do
    context "varint.tjson examples" do
      it "encodes examples successfully" do
        VarintExample.load_file.each do |ex|
          next unless ex.success
          expect(described_class.encode(ex.value)).to eq ex.encoded
        end
      end
    end

    it "raises TypeError if given a non-Integer" do
      expect { described_class.encode(0.5) }.to raise_error TypeError
    end

    it "raises ArgumentError if given a negative value" do
      expect { described_class.encode(-1) }.to raise_error ArgumentError
    end

    it "raises ArgumentError if given Integer is larger than 64-bit unsigned" do
      expect { described_class.encode(Zser::Varint::MAX + 1) }.to raise_error ArgumentError
    end
  end

  describe ".decode" do
    it "decodes examples successfully" do
      VarintExample.load_file.each do |ex|
        if ex.success
          expect(described_class.decode(ex.encoded)).to eq [ex.value, ""]
        else
          expect { described_class.decode(ex.encoded) }.to raise_error Zser::ParseError
        end
      end
    end

    it "raises ArgumentError on an empty string" do
      expect { described_class.decode("") }.to raise_error ArgumentError
    end

    it "raises TypeError if given a non-string type" do
      expect { described_class.decode(42) }.to raise_error TypeError
    end

    it "raises Zser::TruncatedMessageError if input is truncated" do
      expect { described_class.decode("\x02") }.to raise_error Zser::TruncatedMessageError
      expect { described_class.decode("\x00\xFF") }.to raise_error Zser::TruncatedMessageError
    end
  end
end
