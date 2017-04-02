import { suite, test } from "mocha-typescript";
import { expect } from "chai";
import { Varint } from "../src/varint";

@suite class VarintEncode {
  @test "encodes valid examples"() {
    // 0
    expect(Varint.encode(0)).to.eql(new Uint8Array([0x1]));

    // 42
    expect(Varint.encode(42)).to.eql(new Uint8Array([0x55]));

    // 127
    expect(Varint.encode(127)).to.eql(new Uint8Array([0xFF]));

    // 128
    expect(Varint.encode(128)).to.eql(new Uint8Array([0x2, 0x2]));

    // 2**30-1 MAX (presently capped by JS integer precision)
    // TODO: support full 64-bit range when ECMAScript adds support
    expect(Varint.encode(Varint.MAX)).to.eql(new Uint8Array([0xF0, 0xFF, 0xFF, 0xFF, 0x7]));


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
  @test "decodes valid examples"() {
    // 0 with nothing trailing
    expect(Varint.decode(new Uint8Array([0x1]))).to.eql(0);

    // 42 with nothing trailing
    expect(Varint.decode(new Uint8Array([0x55]))).to.eql(42);

    // 127 with nothing trailing
    expect(Varint.decode(new Uint8Array([0xFF]))).to.eql(127);

    // 128 with nothing trailing
    expect(Varint.decode(new Uint8Array([0x2, 0x2]))).to.eql(128);

    // Serialization of Varint.MAX
    expect(Varint.decode(new Uint8Array([0xF0, 0xFF, 0xFF, 0xFF, 0x7]))).to.eql(Varint.MAX);
  }

  @test "throws Error for empty Uint8Array"() {
    expect(() => Varint.decode(new Uint8Array(0))).to.throw(Error);
  }

  @test "throws RangeError for params larger than Varint.MAX"() {
    // Serialization of Varint.MAX + 1
    expect(() => Varint.decode(new Uint8Array([0x10, 0, 0, 0, 8]))).to.throw(RangeError);

    // Serialization of maximum 5-byte zsint
    expect(() => Varint.decode(new Uint8Array([0xF0, 0xFF, 0xFF, 0xFF, 0xFF]))).to.throw(RangeError);
  }
}
