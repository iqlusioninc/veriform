import { suite, test } from "mocha-typescript";
import { expect } from "chai";
import { ZhashExample } from "./support/zhash_examples";
import { Zhash } from "../src/zhash";
import WebCrypto = require("node-webcrypto-ossl");

@suite class ZhashSpec {
  static examples: ZhashExample[];

  static before() {
    return ZhashExample.loadAll(examples => this.examples = examples);
  }

  @test async "encodes valid examples"() {
    let crypto = new WebCrypto();

    for (let example of ZhashSpec.examples) {
      let actual = await Zhash.digest(example.value, example.algorithm, crypto);
      expect(actual).to.eql(example.digest);
    }
  }
}
