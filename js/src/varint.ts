// zsint: Little Endian 64-bit Unsigned Prefix Varints

export class Varint {
  // Maximum value we can represent with a varint before overflowing Math.pow
  // TODO: allow full 64-bit range when ECMAScript adds integers
  public static readonly MAX = Math.pow(2, 30) - 1;

  // Encode a number as a zsint
  public static encode(value: number): Uint8Array {
    if (typeof value !== "number" || (value % 1) !== 0) {
      throw new TypeError(`value ${value} is not an integer`);
    }

    if (value < 0) {
      throw new RangeError("value must be positive or zero");
    }

    if (value > Varint.MAX) {
      throw new RangeError(`value ${value} is too large (maximum ${Varint.MAX})`);
    }

    let length = 1;
    let result = (value << 1) | 1;
    let max = 1 << 7;

    while (value >= max) {
      // Shouldn't be possible, but included as a precaution
      if (max === 0 || max >= Varint.MAX) {
        throw new RangeError(`maximum range overflow: ${max}`);
      }

      // Bitwise shifts are capped at 32-bits
      max *= Math.pow(2, 7);
      result *= Math.pow(2, 1);
      length += 1;
    }

    // Shouldn't be possible, but included as a precaution
    if (result < 0) {
      throw new RangeError(`result overflow: ${result}`);
    }

    let left_half = result % Math.pow(2, 32);
    let right_half = (result - left_half) / Math.pow(2, 32);

    let buffer = new ArrayBuffer(8);
    let view = new DataView(buffer);
    view.setUint32(0, left_half, true);
    view.setUint32(4, right_half, true);

    return new Uint8Array(buffer, 0, length)
  }

  // Decode a serialized zsint
  public static decode(bytes: Uint8Array): number {
    if (!(bytes instanceof Uint8Array)) {
      throw new TypeError("expected a Uint8Array parameter");
    }

    if (bytes.length === 0) {
      throw new Error("cannot decode empty array");
    }

    // TODO: allow full 64-bit range when ECMAScript adds integers
    if (bytes.length > 5) {
      throw new Error("array must be 5 bytes or fewer (due to JS limitations)");
    }

    let prefix = bytes[0];
    let count = 1;

    // Count trailing zeroes
    while ((prefix & 1) === 0) {
      count++;
      prefix >>= 1;
    }

    if (bytes.length != count) {
      throw new Error(`expected ${count} bytes of data, got ${bytes.length}`);
    }

    let buffer = new ArrayBuffer(8);
    new Uint8Array(buffer).set(bytes);
    let view = new DataView(buffer);

    let left_half = view.getUint32(0, true);
    let right_half = view.getUint32(4, true);

    let unshifted_value = right_half * Math.pow(2, 32) + left_half;
    let result = (unshifted_value - (1 << (count - 1))) / Math.pow(2, count);

    if (result > Varint.MAX) {
      throw new RangeError("decoded value is too large (due to JS limitations)");
    }

    return result;
  }
}
