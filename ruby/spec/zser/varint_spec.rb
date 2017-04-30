# encoding: binary
# frozen_string_literal: true

RSpec.describe Zser::Varint do
  describe ".encode" do
    it "encodes valid examples" do
      # 0
      expect(described_class.encode(0)).to eq "\x01"

      # 42
      expect(described_class.encode(42)).to eq "U"

      # 127
      expect(described_class.encode(127)).to eq "\xFF"

      # 128
      expect(described_class.encode(128)).to eq "\x02\x02"

      # 2**64-2
      expect(described_class.encode(18_446_744_073_709_551_614))
        .to eq "\x00\xFE\xFF\xFF\xFF\xFF\xFF\xFF\xFF"

      # 2**64-1
      expect(described_class.encode(18_446_744_073_709_551_615))
        .to eq "\x00\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF"
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
    it "decodes valid examples" do
      # 0 with nothing trailing
      expect(described_class.decode("\x01")).to eq [0, ""]

      # 0 with trailing 0
      expect(described_class.decode("\x01\0")).to eq [0, "\0"]

      # 42 with trailing 0
      expect(described_class.decode("U\0")).to eq [42, "\0"]

      # 127 with trailing 0
      expect(described_class.decode("\xFF\0")).to eq [127, "\0"]

      # 128 with trailing 0
      expect(described_class.decode("\x02\x02\0")).to eq [128, "\0"]

      # 2**64-2 with trailing 0
      expect(described_class.decode("\x00\xFE\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0"))
        .to eq [18_446_744_073_709_551_614, "\0"]

      # 2**64-1 with trailing 0
      expect(described_class.decode("\x00\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0"))
        .to eq [18_446_744_073_709_551_615, "\0"]
    end

    it "raises ArgumentError on an empty string" do
      expect { described_class.decode("") }.to raise_error ArgumentError
    end

    it "raises TypeError if given a non-string type" do
      expect { described_class.decode(42) }.to raise_error TypeError
    end

    it "raises Zser::EOFError if input is truncated" do
      expect { described_class.decode("\x02") }.to raise_error Zser::EOFError
      expect { described_class.decode("\x00\xFF") }.to raise_error Zser::EOFError
    end
  end
end
