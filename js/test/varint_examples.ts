import * as fs from "async-file";
import TJSON from "tjson-js";
import { Varint } from "../src/varint";

export class VarintExample {
  static readonly DEFAULT_EXAMPLES_PATH = "../vectors/varint.tjson";

  static async loadAll(callback: (ex: VarintExample[]) => void) {
    return VarintExample.loadFromFile(VarintExample.DEFAULT_EXAMPLES_PATH, callback);
  }

  static async loadFromFile(filename: string, callback: (ex: VarintExample[]) => void) {
    let tjsonString = await fs.readFile(filename, "utf8");
    let tjson = TJSON.parse(VarintExample.filterUnsupported(tjsonString));
    let examples = tjson["examples"];

    callback(examples.map((ex: any) => VarintExample.decode(ex)));
  }

  // Filter out-of-range examples from vectors/varint.tjson
  // TODO: support full 64-bit range when TC39 BigInt is available
  private static filterUnsupported(tjsonString: string): string {
    let json = JSON.parse(tjsonString);
    let examples = json["examples:A<O>"].filter((example: any) => {
      return parseInt(example["value:u"]) <= Varint.MAX;
    });

    return JSON.stringify({ "examples:A<O>": examples });
  }

  static decode(tjson: any) {
    let ex = Object.create(VarintExample.prototype);
    return Object.assign(ex, tjson);
  }

  constructor(
    public readonly value: number,
    public readonly encoded: Uint8Array,
  ) { }
}
