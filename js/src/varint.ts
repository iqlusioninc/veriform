// varint.ts: Little Endian 64-bit Unsigned Prefix Varints

import { Uint64 } from "./uint64";

/**
 * Variable-width integers (varints) provide a compact wire representation
 * of an integer with a relatively simple encoding.
 *
 * "Prefix Varints" are a different encoding than the one used in
 * Protocol Buffers (LEB128) which provides a simple, loop-free decode step
 * and a natural way to represent 64-bit integers in a max of 9-bytes.
 */
export class Varint {
  /**
   * Maximum allowed integer value
   *
   * TODO: allow full 64-bit range when TC39 BigInts are available:
   * https://tc39.github.io/proposal-bigint/
   */
  public static readonly MAX = Math.pow(2, 53) - 1;

  /** Encode a safe JavaScript integer as a zsint */
  public static encode(n: number): Uint8Array {
    if (typeof n !== "number" || (n % 1) !== 0) {
      throw new TypeError(`value ${n} is not an integer`);
    }

    const result = Uint64.fromNumber(n).bitwiseLeftShift(1).bitwiseOr(1);
    const max = Uint64.fromNumber(1 << 7);
    let length = 1;

    while (max.lessThanOrEqual(n)) {
      result.bitwiseLeftShift(1);
      max.bitwiseLeftShift(7);
      length += 1;
    }

    const buffer = new ArrayBuffer(8);
    const view = new DataView(buffer);
    view.setUint32(0, result.lower(), true);
    view.setUint32(4, result.upper(), true);

    return new Uint8Array(buffer, 0, length);
  }

  /** Decode a serialized zsint, returning its value and any remaining data */
  public static decode(bytes: Uint8Array): [number, Uint8Array] {
    if (!(bytes instanceof Uint8Array)) {
      throw new TypeError("expected a Uint8Array parameter");
    }

    if (bytes.length === 0) {
      throw new Error("cannot decode empty array");
    }

    const prefix = bytes[0];

    // Determine number of trailing zeroes using CTZ_TABLE
    const length = Varint.CTZ_TABLE[prefix] + 1;

    if (bytes.length < length) {
      throw new Error(`not enough bytes in buffer (expected ${length}, got ${bytes.length}`);
    }

    const buffer = new ArrayBuffer(8);
    new Uint8Array(buffer).set(bytes.subarray(0, length));
    const view = new DataView(buffer);

    const values = new Uint32Array(2);
    values[0] = view.getUint32(0, true);
    values[1] = view.getUint32(4, true);

    // This will throw an exception if we're outside the safe integer range
    const result = new Uint64(values).bitwiseRightShift(length).toInteger();
    const remaining = bytes.subarray(length);

    return [result, remaining];
  }

  /** Number of trailing zeros in a given byte value */
  private static readonly CTZ_TABLE = new Uint8Array([
    8, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    6, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    7, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    6, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
  ]);
}
