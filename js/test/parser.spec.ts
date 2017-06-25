import { suite, test } from "mocha-typescript";
import { expect } from "chai";
import { MessageExample } from "./support/test_vectors";
import { Decoder } from "../src/decoder";
import { Parser } from "../src/parser";

@suite class ParserSpec {
  static examples: MessageExample[];

  static async before() {
    this.examples = await MessageExample.loadAll();
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
