// zhash.ts: Structured hashing inspired by Merkle trees

// One character "tag" values used to separate zhash domains
namespace Tags {
  // "Objects" represent veriform messages
  export const OBJECT = "O";

  // 8-bit clean binary data
  export const BINARY = "d";

  // 64-bit unsigned integers
  export const UINT64 = "u";
}

// By default we compute zhashes using SHA-256
const DEFAULT_HASH_ALGORITHM = "SHA-256";

// Number of bytes in a digest, for all supported hash functions
const DIGEST_SIZE = 32;

export class Zhash {
  public static async digest(
    obj: object,
    algorithm = DEFAULT_HASH_ALGORITHM,
    crypto = window.crypto,
  ): Promise<Uint8Array> {
    return (new Zhash(algorithm, crypto)).digest(obj);
  }

  private readonly algorithm: string;

  constructor(
    algorithm = DEFAULT_HASH_ALGORITHM,
    private readonly crypto = window.crypto,
  ) {
    switch (algorithm) {
      // Map from the Veriform algorithm identifier strings to WebCrypto's
      case "SHA256":
        this.algorithm = "SHA-256";
        break;
      default:
        throw new Error(`invalid algorithm identifier: ${algorithm}`);
    }
  }

  public async digest(obj: any): Promise<Uint8Array> {
    if (obj instanceof Uint8Array) {
      return this.digestBinary(obj);
    } else if (typeof obj === "object") {
      return this.digestObject(obj);
    } else if (typeof obj === "number") {
      // TODO: switch to TC39 BigInt
      return this.digestUint64(obj);
    } else {
      throw new TypeError(`can't compute zhash of ${typeof obj}`);
    }
  }

  private async digestObject(obj: any): Promise<Uint8Array> {
    const arr = new Uint8Array(1 + Object.keys(obj).length * (DIGEST_SIZE + 8));
    arr[0] = Tags.OBJECT.charCodeAt(0);
    let offset = 1;

    for (const key of Object.keys(obj).sort().map((e) => Number(e))) {
      arr.set(encodeUint64(Number(key)), offset);
      offset += 8;

      arr.set(await this.digest(obj[key]), offset);
      offset += DIGEST_SIZE;
    }

    const buffer = await this.crypto.subtle.digest(this.algorithm, arr.buffer);
    return new Uint8Array(buffer);
  }

  private async digestBinary(bytes: Uint8Array) {
    return this.taggedDigest(Tags.BINARY, bytes);
  }

  // TODO: switch to TC39 BigInt
  private async digestUint64(num: number) {
    return this.taggedDigest(Tags.UINT64, encodeUint64(num));
  }

  private async taggedDigest(tag: string, value: ArrayLike<number>): Promise<Uint8Array> {
    const arr = new Uint8Array(1 + value.length);
    arr[0] = tag.charCodeAt(0);
    arr.set(value, 1);

    const buffer = await this.crypto.subtle.digest(this.algorithm, arr.buffer);
    return new Uint8Array(buffer);
  }
}

function encodeUint64(num: number): Uint8Array {
  const buffer = new ArrayBuffer(8);
  const view = new DataView(buffer);
  view.setUint32(0, num & 0xFFFFFFFF, true);
  view.setUint32(4, (num & 0x1fffff00000000) >>> 32, true);
  return new Uint8Array(buffer);
}
