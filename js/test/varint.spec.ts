import { suite, test } from "mocha-typescript";
import { expect } from "chai";
import { Varint, Uint64 } from "../src/varint";

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

    // 2**53-1 MAX (presently capped by JS integer precision)
    // TODO: support full 64-bit range when ECMAScript adds support
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
    expect(Varint.decode(new Uint8Array([0x80, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x1F]))).to.eql(Varint.MAX);
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
    /// Low-value example
    expect(Uint64.fromNumber(128).lshift(1).toNumber()).to.eql(256);

    /// this.exampleNumber >> 1
    expect(Uint64.fromNumber(264140488932).lshift(1).toNumber()).to.eql(this.exampleNumber);

    // this.exampleNumber >> 2
    expect(Uint64.fromNumber(132070244466).lshift(2).toNumber()).to.eql(this.exampleNumber);
  }

  @test "right bitwise shift"() {
    expect(Uint64.fromNumber(this.exampleNumber).rshift(1).toNumber()).to.eql(264140488932);
    expect(Uint64.fromNumber(this.exampleNumber).rshift(2).toNumber()).to.eql(132070244466);
  }

  @test "bitwise OR"() {
    expect(Uint64.fromNumber(256).bw_or(1).toNumber()).to.eql(257);
    expect(Uint64.fromNumber(this.exampleNumber).bw_or(7).toNumber()).to.eql(528280977871);
  }

  @test "less than or equal"() {
    expect(this.exampleUint.lt_eq(528280977865)).to.be.true;
    expect(this.exampleUint.lt_eq(528280977864)).to.be.true;
    expect(this.exampleUint.lt_eq(528280977863)).to.be.false;
  }

  @test "upper and lower"() {
    expect(this.exampleUint.upper()).to.eql(123);
    expect(this.exampleUint.lower()).to.eql(456);
  }
}
