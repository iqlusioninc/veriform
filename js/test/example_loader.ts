import * as fs from "async-file";
import TJSON from "tjson-js";

export class Example {
  static decode(tjson: any) {
    let ex = Object.create(Example.prototype);
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

export class ExampleLoader {
  static readonly DEFAULT_EXAMPLES_PATH = "../vectors/messages.tjson";

  examplesFile: string;

  constructor(file = ExampleLoader.DEFAULT_EXAMPLES_PATH) {
    this.examplesFile = file;
  }

  async load(callback: (ex: Example[]) => void) {
    let tjsonString = await fs.readFile(this.examplesFile, "utf8");
    let tjson = TJSON.parse(tjsonString);
    let examples = tjson["examples"];
    callback(examples.map((ex: any) => Example.decode(ex)));
  }
}
