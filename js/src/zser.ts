import { Decoder } from "./decoder";
import { Parser } from "./parser";

export default class Zser {
  // Parse a Uint8Array containing an encoded zser message to an object representation
  public static parse(message: Uint8Array): object {
    const parser = new Parser(new Decoder());
    parser.parse(message);
    return parser.finish();
  }
}
