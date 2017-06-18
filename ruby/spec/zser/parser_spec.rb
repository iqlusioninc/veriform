# frozen_string_literal: true

RSpec.describe Zser::Parser do
  context "message.tjson examples" do
    # Integration test against the decoder
    subject(:parser) { described_class.new(Zser::Decoder.new) }

    MessageExample.load_file.each do |example|
      it example.name do
        if example.success
          expect(parser.parse(example.encoded)).to eq true
          result = parser.finish
          expect(result).to be_a Zser::Object
          expect(result).to eql Zser::Object.from_tjson(example.decoded)
        else
          expect { parser.parse(example.encoded) }.to raise_error Zser::ParseError
        end
      end
    end
  end
end
