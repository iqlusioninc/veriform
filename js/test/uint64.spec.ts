import { suite, test } from "mocha-typescript";
import { expect } from "chai";
import { Uint64 } from "../src/uint64";

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
