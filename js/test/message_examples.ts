import * as fs from "async-file";
import TJSON from "tjson-js";

export class MessageExample {
  static readonly DEFAULT_EXAMPLES_PATH = "../vectors/messages.tjson";

  static async loadAll(callback: (ex: MessageExample[]) => void) {
    return MessageExample.loadFromFile(MessageExample.DEFAULT_EXAMPLES_PATH, callback);
  }

  static async loadFromFile(filename: string, callback: (ex: MessageExample[]) => void) {
    let tjsonString = await fs.readFile(filename, "utf8");
    let tjson = TJSON.parse(tjsonString);
    let examples = tjson["examples"];
    callback(examples.map((ex: any) => MessageExample.decode(ex)));
  }

  static decode(tjson: any) {
    let ex = Object.create(MessageExample.prototype);
    return Object.assign(ex, tjson);
  }

  constructor(
    public readonly name: string,
    public readonly description: string,
    public readonly success: boolean,
    public readonly encoded: Uint8Array,
    public readonly decoded: object | undefined
  ) { }
}
