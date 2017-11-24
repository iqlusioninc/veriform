import { suite, test } from "mocha-typescript";
import { expect } from "chai";
import { VerihashExample } from "./support/test_vectors";
import { Verihash } from "../src/verihash";
import WebCrypto = require("node-webcrypto-ossl");

@suite class VerihashSpec {
  static examples: VerihashExample[];

  static async before() {
    this.examples = await VerihashExample.loadAll();
  }

  @test async "encodes valid examples"() {
    let crypto = new WebCrypto();

    for (let example of VerihashSpec.examples) {
      let actual = await Verihash.digest(example.value, example.algorithm, crypto);
      expect(actual).to.eql(example.digest);
    }
  }
}
