import { suite, test } from "mocha-typescript";
import { expect } from "chai";
import { Example, ExampleLoader } from "./example_loader";
import { Decoder } from "../src/decoder";
import { Parser } from "../src/parser";

@suite class ParserSpec {
  static examples: Example[];

  static before() {
    return (new ExampleLoader).load(examples => this.examples = examples);
  }

  @test "vectors/messages.tjson (zser message test vectors)"() {
    for (let example of ParserSpec.examples) {
      let parser = new Parser(new Decoder);

      if (example.success) {
        parser.parse(example.encoded);
        expect(parser.finish()).to.eql(example.decoded);
      } else {
        expect(() => parser.parse(example.encoded)).to.throw;
      }
    }
  }
}
