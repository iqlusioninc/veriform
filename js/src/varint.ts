// zsint: Little Endian 64-bit Unsigned Prefix Varints

export class Varint {
  // Maximum allowed integer value
  //
  // TODO: allow full 64-bit range when TC39 integers are available:
  // https://github.com/tc39/proposal-integer
  public static readonly MAX = Math.pow(2, 53) - 1;

  // Number of trailing zeros in a given byte value
  static readonly CTZ_TABLE = new Uint8Array([
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
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0
  ]);

  // Encode a safe JavaScript integer as a zsint
  public static encode(n: number): Uint8Array {
    if (typeof n !== "number" || (n % 1) !== 0) {
      throw new TypeError(`value ${n} is not an integer`);
    }

    let result = Uint64.fromNumber(n).lshift(1).bw_or(1);
    let max = Uint64.fromNumber(1 << 7);
    let length = 1;

    while (max.lt_eq(n)) {
      result.lshift(1);
      max.lshift(7);
      length += 1;
    }

    let buffer = new ArrayBuffer(8);
    let view = new DataView(buffer);
    view.setUint32(0, result.lower(), true);
    view.setUint32(4, result.upper(), true);

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

    // TODO: allow full 64-bit range when ECMAScript adds integers/bignums
    if (bytes.length > 8) {
      throw new RangeError("array must be 8 bytes or fewer (due to JS limitations)");
    }

    let prefix = bytes[0];

    // Determine number of trailing zeroes using CTZ_TABLE
    let count = Varint.CTZ_TABLE[prefix] + 1;

    if (bytes.length != count) {
      throw new Error(`expected ${count} bytes of data, got ${bytes.length}`);
    }

    let buffer = new ArrayBuffer(8);
    new Uint8Array(buffer).set(bytes);
    let view = new DataView(buffer);

    let values = new Uint32Array(2);
    values[0] = view.getUint32(0, true);
    values[1] = view.getUint32(4, true);

    return new Uint64(values).rshift(count).toInteger();
  }
}

// A Uint64 represented as two 32-bit uints, with the bitwise ops we need
// to implement zsints. This allows us to do bitwise arithmetic that is
// outside the MAX_SAFE_INTEGER range.
//
// TODO: remove when we can use TC39 integers: https://github.com/tc39/proposal-integer
export class Uint64 {
  values: Uint32Array;

  public static readonly MAX_SAFE_INTEGER = Math.pow(2, 53) - 1;

  public static fromNumber(n: number): Uint64 {
    Uint64.checkInteger(n);

    let values = new Uint32Array(2);
    values[0] = n & 0xFFFFFFFF;
    values[1] = (n - values[0]) / Math.pow(2, 32);
    return new Uint64(values);
  }

  // Is this number an integer in the safe range?
  static checkInteger(n: number) {
    if (n < 0) {
      throw new RangeError("number must be positive");
    }

    if (n > Uint64.MAX_SAFE_INTEGER) {
      throw new RangeError("number is outside the safe integer range");
    }
  }

  constructor(values: Uint32Array) {
    if (values.length !== 2) {
      throw new TypeError("argument must be a Uint32Array(2)");
    }

    this.values = values;
  }

  // Bitwise left shift. Overflows are silently ignored.
  public lshift(n: number): Uint64 {
    if (n < 0) {
      throw new RangeError("number must be positive");
    }

    if (n > 32) {
      throw new RangeError("can only shift 32-bits at a time");
    }

    let carry = this.values[0] >> (32 - n) & ((1 << n) - 1);
    this.values[1] = (this.values[1] << n | carry) & 0xFFFFFFFF;
    this.values[0] = (this.values[0] << n) & 0xFFFFFFFF;

    return this;
  }

  // Bitwise shift right
  public rshift(n: number): Uint64 {
    if (n < 0) {
      throw new RangeError("number must be positive");
    }

    if (n > 32) {
      throw new RangeError("can only shift 32-bits at a time");
    }

    let carry = this.values[1] & ((1 << n) - 1);
    this.values[1] >>= n;
    this.values[0] = (this.values[0] >> n) | (carry << (32 - n));

    return this;
  }

  // Bitwise OR. Value must be in the 32-bit range
  public bw_or(n: number): Uint64 {
    if (n < 0) {
      throw new RangeError("number must be positive");
    }

    if (n > 0xFFFFFFFF) {
      throw new RangeError("value must be in the 32-bit range");
    }

    this.values[0] |= n;

    return this;
  }

  // Is this value less than or equal to the given integer?
  public lt_eq(n: number) {
    Uint64.checkInteger(n);

    let nLower = n & 0xFFFFFFFF;
    let nUpper = (n - nLower) / Math.pow(2, 32);

    if (this.values[1] < nUpper) {
      return true;
    } else if (this.values[1] === nUpper) {
      return this.values[0] <= nLower;
    } else {
      return false;
    }
  }

  // Upper 32-bits
  public upper(): number {
    return this.values[1];
  }

  // Lower 32-bits
  public lower(): number {
    return this.values[0];
  }

  // Convert to a safe JavaScript integer (or throw RangeError if not possible)
  public toInteger(): number {
    if (this.values[1] > 2097151) {
      throw new RangeError("value is outside MAX_SAFE_INTEGER range");
    }

    return (this.values[1] * Math.pow(2, 32)) + this.values[0];
  }
}
