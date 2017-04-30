import { suite, test } from "mocha-typescript";
import { expect } from "chai";
import { Decoder } from "../src/decoder";

@suite class DecoderSpec {
  @test "uint64"() {
    let decoder = new Decoder();
    decoder.uint64(1, 42);
    expect(decoder.finish()[1]).to.eq(42);
  }

  @test "binary"() {
    let decoder = new Decoder();
    let exampleBytes = new Uint8Array([1, 2, 3]);

    decoder.binary(42, exampleBytes);
    expect(decoder.finish()[42]).to.eq(exampleBytes);
  }
}
