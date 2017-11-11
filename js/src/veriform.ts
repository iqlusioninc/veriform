import { Decoder } from "./decoder";
import { Parser } from "./parser";

export default class Veriform {
  /** Parse a Uint8Array containing an encoded Veriform message to an object representation */
  public static parse(message: Uint8Array): object {
    const parser = new Parser(new Decoder());
    parser.parse(message);
    return parser.finish();
  }
}
