# frozen_string_literal: true

RSpec.describe Veriform::Parser do
  context "message.tjson examples" do
    # Integration test against the decoder
    subject(:parser) { described_class.new(Veriform::Decoder.new) }

    MessageExample.load_file.each do |example|
      it example.name do
        if example.success
          expect(parser.parse(example.encoded)).to eq true
          result = parser.finish
          expect(result).to be_a Veriform::Object
          expect(result).to eql Veriform::Object.from_tjson(example.decoded)
        else
          expect { parser.parse(example.encoded) }.to raise_error Veriform::ParseError
        end
      end
    end
  end
end
