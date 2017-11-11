# frozen_string_literal: true

RSpec.describe Veriform::Decoder do
  subject(:decoder) { described_class.new }

  describe "#uint64" do
    it "adds a uint64 field to the resulting object" do
      decoder.uint64(1, 42)
      expect(decoder.finish[1]).to eq 42
    end
  end

  describe "#binary" do
    it "deserializes binary data"
  end

  context "nested messages" do
    describe "#begin_nested" do
      it "begins a new nested message"
    end

    describe "#end_nested" do
      it "adds the nested object to the current message"
    end
  end
end
