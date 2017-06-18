/**
 * A Uint64 represented as two 32-bit uints, with the bitwise ops we need
 * to implement zsints. This allows us to do bitwise arithmetic that is
 * outside the MAX_SAFE_INTEGER range.
 *
 * TODO: remove when we can use TC39 BigInt: https://tc39.github.io/proposal-bigint/
 */
export class Uint64 {
  public static readonly MAX_SAFE_INTEGER = Math.pow(2, 53) - 1;

  public static fromNumber(n: number): Uint64 {
    Uint64.checkInteger(n);

    let values = new Uint32Array(2);
    values[0] = n & 0xFFFFFFFF;
    values[1] = (n - values[0]) / Math.pow(2, 32);
    return new Uint64(values);
  }

  /** Is this number an integer in the safe range? */
  public static checkInteger(n: number) {
    if (n < 0) {
      throw new RangeError("number must be positive");
    }

    if (n > Uint64.MAX_SAFE_INTEGER) {
      throw new RangeError("number is outside the safe integer range");
    }
  }

  constructor(private values: Uint32Array) {
    if (values.length !== 2) {
      throw new TypeError("argument must be a Uint32Array(2)");
    }
  }

  /** Bitwise left shift. Overflows are silently ignored. */
  public bitwiseLeftShift(n: number): Uint64 {
    if (n < 0) {
      throw new RangeError("number must be positive");
    }

    if (n > 32) {
      throw new RangeError("can only shift 32-bits at a time");
    }

    let carry = this.values[0] >>> (32 - n) & ((1 << n) - 1);
    this.values[1] = (this.values[1] << n | carry) & 0xFFFFFFFF;
    this.values[0] = (this.values[0] << n) & 0xFFFFFFFF;

    return this;
  }

  /** Bitwise shift right */
  public bitwiseRightShift(n: number): Uint64 {
    if (n < 0) {
      throw new RangeError("number must be positive");
    }

    if (n > 32) {
      throw new RangeError("can only shift 32-bits at a time");
    }

    let carry = this.values[1] & ((1 << n) - 1);
    this.values[1] >>>= n;
    this.values[0] = (this.values[0] >>> n) | (carry << (32 - n));

    return this;
  }

  /** Bitwise OR. Value must be in the 32-bit range */
  public bitwiseOr(n: number): Uint64 {
    if (n < 0) {
      throw new RangeError("number must be positive");
    }

    if (n > 0xFFFFFFFF) {
      throw new RangeError("value must be in the 32-bit range");
    }

    this.values[0] |= n;

    return this;
  }

  /** Is this value less than or equal to the given integer? */
  public lessThanOrEqual(n: number) {
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

  /** Upper 32-bits */
  public upper(): number {
    return this.values[1];
  }

  /** Lower 32-bits */
  public lower(): number {
    return this.values[0];
  }

  /** Convert to a safe JavaScript integer (or throw RangeError if not possible) */
  public toInteger(): number {
    if (this.values[1] > 2097151) {
      throw new RangeError("value is outside MAX_SAFE_INTEGER range");
    }

    return (this.values[1] * Math.pow(2, 32)) + this.values[0];
  }
}
