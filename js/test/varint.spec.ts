import { suite, test } from "mocha-typescript";
import { expect } from "chai";
import { Varint } from "../src/varint";
import { VarintExample } from "./varint_examples";

@suite class VarintEncode {
  static examples: VarintExample[];

  static before() {
    return VarintExample.loadAll(examples => this.examples = examples);
  }

  @test "encodes valid examples"() {
    for (let example of VarintEncode.examples) {
      if (!example.success) {
        continue;
      }

      expect(Varint.encode(example.value)).to.eql(example.encoded);
    }

    // 2**53-1 MAX (presently capped by JS integer precision)
    expect(Varint.encode(Varint.MAX)).to.eql(new Uint8Array([0x80, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F]));
  }

  @test "throws TypeError for non-integer param"() {
    expect(() => Varint.encode(0.5)).to.throw(TypeError);
  }

  @test "throws RangeError for negative param"() {
    expect(() => Varint.encode(-1)).to.throw(RangeError);
  }

  @test "throws RangeError for param larger than Varint.MAX"() {
    expect(() => Varint.encode(Varint.MAX + 1)).to.throw(RangeError);
  }
}

@suite class VarintDecode {
  static examples: VarintExample[];

  static before() {
    return VarintExample.loadAll(examples => this.examples = examples);
  }

  @test "decodes valid examples"() {
    for (let example of VarintEncode.examples) {
      if (example.success) {
        expect(Varint.decode(example.encoded)[0]).to.eql(example.value);
      } else {
        expect(() => Varint.decode(example.encoded)).to.throw(Error);
      }
    }

    // Serialization of Varint.MAX
    expect(Varint.decode(new Uint8Array([0x80, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F]))[0]).to.eql(Varint.MAX);
  }

  @test "throws Error for empty Uint8Array"() {
    expect(() => Varint.decode(new Uint8Array(0))).to.throw(Error);
  }

  @test "throws RangeError for params larger than Varint.MAX"() {
    // Serialization of Varint.MAX + 1
    expect(() => Varint.decode(new Uint8Array([0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32]))).to.throw(RangeError);
  }
}
