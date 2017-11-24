import * as fs from "async-file";
import TJSON from "tjson-js";
import { Varint } from "../../src/varint";

export class MessageExample {
  static readonly DEFAULT_EXAMPLES_PATH = "../vectors/messages.tjson";

  static async loadAll(): Promise<MessageExample[]> {
    return MessageExample.loadFromFile(MessageExample.DEFAULT_EXAMPLES_PATH);
  }

  static async loadFromFile(filename: string): Promise<MessageExample[]> {
    let tjson = TJSON.parse(await fs.readFile(filename, "utf8"));
    return tjson["examples"].map((ex: any) => {
      let obj = Object.create(MessageExample.prototype);
      return Object.assign(obj, ex);
    });
  }

  constructor(
    public readonly name: string,
    public readonly description: string,
    public readonly success: boolean,
    public readonly encoded: Uint8Array,
    public readonly decoded: object | undefined
  ) { }
}

export class VarintExample {
  static readonly DEFAULT_EXAMPLES_PATH = "../vectors/varint.tjson";

  static async loadAll(): Promise<VarintExample[]> {
    return VarintExample.loadFromFile(VarintExample.DEFAULT_EXAMPLES_PATH);
  }

  static async loadFromFile(filename: string): Promise<VarintExample[]> {
    let tjsonString = await fs.readFile(filename, "utf8");
    let tjson = TJSON.parse(filterUnsupported(tjsonString));
    return tjson["examples"].map((ex: any) => {
      let obj = Object.create(VarintExample.prototype);
      return Object.assign(obj, ex);
    });
  }

  constructor(
    public readonly value: number,
    public readonly encoded: Uint8Array,
    public readonly success: boolean
  ) { }
}

export class VerihashExample {
  static readonly DEFAULT_EXAMPLES_PATH = "../vectors/verihash.tjson";

  static async loadAll(): Promise<VerihashExample[]> {
    return VerihashExample.loadFromFile(VerihashExample.DEFAULT_EXAMPLES_PATH);
  }

  static async loadFromFile(filename: string): Promise<VerihashExample[]> {
    let tjsonString = await fs.readFile(filename, "utf8");
    let tjson = TJSON.parse(filterUnsupported(tjsonString));
    return tjson["examples"].map((ex: any) => {
      let obj = Object.create(VerihashExample.prototype);
      return Object.assign(obj, ex);
    });
  }

  constructor(
    public readonly name: string,
    public readonly algorithm: string,
    public readonly digest: Uint8Array,
    public readonly value: object
  ) { }
}

// Filter out-of-range examples from vectors/varint.tjson
// TODO: support full 64-bit range when TC39 BigInt is available
function filterUnsupported(tjsonString: string): string {
  let json = JSON.parse(tjsonString);
  let examples = json["examples:A<O>"].filter((example: any) => {
    return parseInt(example["value:u"]) <= Varint.MAX;
  });

  return JSON.stringify({ "examples:A<O>": examples });
}
