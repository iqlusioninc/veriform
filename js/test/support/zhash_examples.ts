import * as fs from "async-file";
import TJSON from "tjson-js";
import { Varint } from "../../src/varint";

export class ZhashExample {
  static readonly DEFAULT_EXAMPLES_PATH = "../vectors/zhash.tjson";

  static async loadAll(callback: (ex: ZhashExample[]) => void) {
    return ZhashExample.loadFromFile(ZhashExample.DEFAULT_EXAMPLES_PATH, callback);
  }

  static async loadFromFile(filename: string, callback: (ex: ZhashExample[]) => void) {
    let tjsonString = await fs.readFile(filename, "utf8");
    let tjson = TJSON.parse(ZhashExample.filterUnsupported(tjsonString));
    let examples = tjson["examples"];

    callback(examples.map((ex: any) => ZhashExample.decode(ex)));
  }

  // Filter out-of-range examples from vectors/zhash.tjson
  // TODO: support full 64-bit range when TC39 BigInt is available
  private static filterUnsupported(tjsonString: string): string {
    let json = JSON.parse(tjsonString);
    let examples = json["examples:A<O>"].filter((example: any) => {
      if ("value:u" in example) {
        return parseInt(example["value:u"]) <= Varint.MAX;
      } else {
        return true;
      }
    });

    return JSON.stringify({ "examples:A<O>": examples });
  }

  static decode(tjson: any) {
    let ex = Object.create(ZhashExample.prototype);
    return Object.assign(ex, tjson);
  }

  constructor(
    public readonly name: string,
    public readonly algorithm: string,
    public readonly digest: Uint8Array,
    public readonly value: object
  ) { }
}
