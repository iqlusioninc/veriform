import { suite, test } from "mocha-typescript";
import { expect } from "chai";
import { Varint, Uint64 } from "../src/varint";
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

@suite class Uint64Spec {
  readonly exampleNumber = 528280977864;
  readonly exampleUint = Uint64.fromNumber(this.exampleNumber);

  @test "throws RangeError for negative numbers"() {
    expect(() => Uint64.fromNumber(-1));
  }

  @test "throws RangeError for values larger than Uint64.MAX_SAFE_INTEGER"() {
    expect(() => Uint64.fromNumber(Uint64.MAX_SAFE_INTEGER + 1)).to.throw(RangeError);
  }

  @test "left bitwise shift"() {
    // Low-value example
    expect(Uint64.fromNumber(128).bitwiseLeftShift(1).toInteger()).to.eql(256);

    // this.exampleNumber >>> 1
    expect(Uint64.fromNumber(264140488932).bitwiseLeftShift(1).toInteger()).to.eql(this.exampleNumber);

    // this.exampleNumber >>> 2
    expect(Uint64.fromNumber(132070244466).bitwiseLeftShift(2).toInteger()).to.eql(this.exampleNumber);
  }

  @test "right bitwise shift"() {
    expect(Uint64.fromNumber(this.exampleNumber).bitwiseRightShift(1).toInteger()).to.eql(264140488932);
    expect(Uint64.fromNumber(this.exampleNumber).bitwiseRightShift(2).toInteger()).to.eql(132070244466);

    // Ensure values that exceed the signed 32-bit range are correctly handled
    expect(Uint64.fromNumber(4294967288).bitwiseRightShift(4).toInteger()).to.eql(268435455);
  }

  @test "bitwise OR"() {
    expect(Uint64.fromNumber(256).bitwiseOr(1).toInteger()).to.eql(257);
    expect(Uint64.fromNumber(this.exampleNumber).bitwiseOr(7).toInteger()).to.eql(528280977871);
  }

  @test "less than or equal"() {
    expect(this.exampleUint.lessThanOrEqual(528280977865)).to.be.true;
    expect(this.exampleUint.lessThanOrEqual(528280977864)).to.be.true;
    expect(this.exampleUint.lessThanOrEqual(528280977863)).to.be.false;
  }

  @test "upper and lower"() {
    expect(this.exampleUint.upper()).to.eql(123);
    expect(this.exampleUint.lower()).to.eql(456);
  }
}
