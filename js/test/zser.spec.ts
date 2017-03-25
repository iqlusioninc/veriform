import { suite, test } from "mocha-typescript";
import { expect } from "chai";
import { Zser } from "../src/zser"

@suite class ZserSpec {
  @test "it works"() {
    expect(true).to.equal(true);
  }
}
